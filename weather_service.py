

# weather_service.py
from typing import Any, Optional
import logging
import httpx
from mcp.server.fastmcp import FastMCP

# Initialize FastMCP server
mcp = FastMCP("weather")

# Constants
NWS_API_BASE = "https://api.weather.gov"
# NWS asks for a User-Agent that includes contact info; update this to your email/org.
USER_AGENT = "weather-app/1.0 (you@example.com)"

logger = logging.getLogger("weather_service")
logging.basicConfig(level=logging.INFO)


async def make_nws_request(url: str) -> dict[str, Any] | None:
    """Make a request to the NWS API with proper error handling."""
    headers = {
        "User-Agent": USER_AGENT,
        "Accept": "application/geo+json",
    }
    async with httpx.AsyncClient() as client:
        try:
            resp = await client.get(url, headers=headers, timeout=30.0)
            resp.raise_for_status()
            return resp.json()
        except httpx.HTTPStatusError as exc:
            logger.warning("NWS returned non-2xx for %s: %s", url, exc)
            return None
        except httpx.RequestError as exc:
            logger.exception("Network error while requesting %s", url)
            return None
        except ValueError:
            logger.exception("Invalid JSON from %s", url)
            return None


def format_alert(feature: dict) -> str:
    """Format an alert feature into a readable string."""
    props = feature.get("properties", {})
    return (
        f"Event: {props.get('event', 'Unknown')}\n"
        f"Area: {props.get('areaDesc', 'Unknown')}\n"
        f"Severity: {props.get('severity', 'Unknown')}\n\n"
        f"Description:\n{props.get('description', 'No description available')}\n\n"
        f"Instructions:\n{props.get('instruction', 'No specific instructions provided')}\n"
    )


def validate_lat_lon(lat: float, lon: float) -> bool:
    return -90.0 <= lat <= 90.0 and -180.0 <= lon <= 180.0


@mcp.get("/")
async def root() -> dict[str, Any]:
    """Health / info endpoint."""
    return {
        "status": "ok",
        "service": "weather",
        "notes": "This service proxies NOAA/NWS (api.weather.gov). Provide lat & lon as query params.",
        "usage": "/weather?lat={lat}&lon={lon}[&hourly=true]",
    }


@mcp.get("/weather")
async def get_weather(lat: float, lon: float, hourly: Optional[bool] = False) -> dict[str, Any]:
    """
    Fetch point metadata, forecast (or hourly forecast), and active alerts for a latitude/longitude.

    Query params:
      - lat: float (required)
      - lon: float (required)
      - hourly: bool (optional) -> use forecastHourly when true
    """
    # Basic validation
    if not validate_lat_lon(lat, lon):
        return {"error": "invalid lat/lon; lat must be -90..90 and lon -180..180"}

    # 1) Query points endpoint to get forecast URLs and metadata
    point_url = f"{NWS_API_BASE}/points/{lat},{lon}"
    logger.info("Fetching point metadata: %s", point_url)
    point = await make_nws_request(point_url)
    if point is None:
        return {"error": "failed to fetch point metadata from NWS"}

    props = point.get("properties", {})
    # pick forecast url
    forecast_url = props.get("forecastHourly") if hourly else props.get("forecast")
    if not forecast_url:
        # fallback: sometimes forecast is present only as forecast (non-hourly)
        forecast_url = props.get("forecast") or props.get("forecastHourly")

    # 2) Fetch forecast (if available)
    forecast = None
    if forecast_url:
        logger.info("Fetching forecast: %s", forecast_url)
        forecast = await make_nws_request(forecast_url)
        if forecast is None:
            logger.warning("Forecast lookup failed for %s", forecast_url)

    # 3) Fetch active alerts for the point
    alerts_url = f"{NWS_API_BASE}/alerts/active?point={lat},{lon}"
    logger.info("Fetching alerts: %s", alerts_url)
    alerts = await make_nws_request(alerts_url)
    formatted_alerts = []
    if alerts and isinstance(alerts.get("features"), list):
        formatted_alerts = [format_alert(f) for f in alerts["features"]]

    # 4) Prepare a small summary (lightweight, user-friendly)
    location_name = None
    rel_loc = props.get("relativeLocation")
    if rel_loc:
        location_name = rel_loc.get("properties", {}).get("city")

    # Build response
    return {
        "point_url": point_url,
        "point": props,          # full point properties (may be large)
        "location_name": location_name,
        "forecast_url": forecast_url,
        "forecast": forecast,    # raw NWS forecast JSON (periods etc.)
        "alerts_raw": alerts,    # raw alerts JSON
        "alerts_text": formatted_alerts,
    }

# Optional: dedicated alerts endpoint if you want raw and formatted responses
@mcp.get("/alerts")
async def get_alerts(lat: float, lon: float) -> dict[str, Any]:
    if not validate_lat_lon(lat, lon):
        return {"error": "invalid lat/lon; lat must be -90..90 and lon -180..180"}

    alerts_url = f"{NWS_API_BASE}/alerts/active?point={lat},{lon}"
    alerts = await make_nws_request(alerts_url)
    if alerts is None:
        return {"error": "failed to fetch alerts from NWS"}
    features = alerts.get("features", [])
    return {
        "count": len(features),
        "raw": alerts,
        "formatted": [format_alert(f) for f in features],
    }

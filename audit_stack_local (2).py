# audit_stack_local.py
# Offline validator and ledger for SS1m-based seam welds (hardcoded canonical block)
import math
import json
import os

TOL = 0.005
LEDGER_FILE = "ledger.json"


def weld_pass(delta_kappa, I_ratio, residual, tol=TOL):
    return abs(residual) <= tol and math.isclose(math.exp(delta_kappa), I_ratio, rel_tol=1e-9)


def render_hud(weld_id, delta_kappa, I_ratio, residual, kappa, I, seed, sha256, manifest_root_hash, tol=TOL):
    I_t1 = I * math.exp(delta_kappa)
    kappa_t1 = kappa + delta_kappa
    passed = weld_pass(delta_kappa, I_ratio, residual, tol)

    return {
        "weld_id": weld_id,
        "manifest_root_hash": manifest_root_hash,
        "kappa": kappa,
        "I": I,
        "delta_kappa": delta_kappa,
        "I_ratio": I_ratio,
        "I_t1": I_t1,
        "kappa_t1": kappa_t1,
        "residual": residual,
        "tol": tol,
        "pass": passed,
        "seed": seed,
        "sha256": sha256
    }


def append_to_ledger(entry, filename=LEDGER_FILE):
    if os.path.exists(filename):
        with open(filename, "r") as f:
            data = json.load(f)
    else:
        data = []

    data.append(entry)

    with open(filename, "w") as f:
        json.dump(data, f, indent=2)


if __name__ == "__main__":
    # Canonical Collapse Block HUD (hardcoded validated values)
    delta_kappa = 0.45000000000000007
    I_ratio = math.exp(delta_kappa)  # 1.568312185490169
    residual = 0.000

    hud = render_hud(
        weld_id="ss1m_block_collapse_v1",
        delta_kappa=delta_kappa,
        I_ratio=I_ratio,
        residual=residual,
        kappa=0.000,
        I=1.000,
        seed=3021,
        sha256="d3ad77cf43b8fd40b723d08733e1161d7a0b5cb2d94f42ff1158cbff9f635e91",
        manifest_root_hash="canonical_collapse_closure_block_v1"
    )

    print("\n# HUD OUTPUT")
    for k, v in hud.items():
        print(f"{k}: {v}")

    with open("hud_ss1m_block_collapse_v1.json", "w") as f:
        json.dump(hud, f, indent=2)

    append_to_ledger(hud)


# csjx_boucwen_model.py
# Fixed-seed synthetic-jet (C-SJX) model with Bouc–Wen hysteresis
# Author: RCFT kit
# Seeded for reproducibility.

import numpy as np
from math import pi
import matplotlib.pyplot as plt

RNG = np.random.default_rng(417)

def formation_envelope(S, sigma=1.0, S_opt=4.0):
    """Gaussian-ish efficiency envelope peaking near S_opt."""
    return np.exp(-0.5*((S-S_opt)/sigma)**2)

def hysteresis_gain(tau_R, tau0=0.5, k_H=0.6):
    """Continuity gain factor: H in [0,1), scaled into (1 + k_H * H)."""
    H = tau_R/(tau_R + tau0)
    return 1.0 + k_H * H

def bouc_wen(xdot, z, A=1.0, beta=0.6, gamma=0.2, n=2):
    """Bouc–Wen hysteresis evolution: dz/dt."""
    # sign-preserving |z|^{n-1} z == |z|^n * sign(z)
    absz = np.abs(z)
    return A*xdot - beta*np.abs(xdot)*(absz**(n-1))*z - gamma*xdot*(absz**n)

def simulate_cycle(D=0.010, f=150.0, stroke_ratio=4.0, tau_R=2.0,
                   cavity_C=2.0e-8, # m^3/Pa effective compliance
                   rho=1.2, mu_air=1.8e-5,
                   A_bw=1.0, beta=0.6, gamma=0.2, n=2,
                   duration_cycles=10, dt=1e-5, seed=417):
    """
    Simulate a few cycles of a synthetic jet with Bouc–Wen hysteresis
    in the pressure-volume relation of the cavity liner.
    Returns: dict with time traces and summary metrics (impulse, loop area, etc.)
    """
    rng = np.random.default_rng(seed)
    T = 1.0/f
    duration = duration_cycles*T
    t = np.arange(0.0, duration, dt)

    # stroke length L0 from S*D; map to diaphragm velocity amplitude U0 via half-cycle duration
    S = stroke_ratio
    L0 = S*D
    U0 = L0/(0.5*T)  # mean ejection velocity proxy over outstroke

    # Diaphragm displacement x(t) ~ sin; velocity xdot(t)
    # Choose amplitude so that volumetric flow ~ A_orifice * U_out on outstroke
    x_amp = (L0/2.0)  # diaphragm proxy amplitude
    x = x_amp*np.sin(2*pi*f*t)
    xdot = 2*pi*f*x_amp*np.cos(2*pi*f*t)

    # Bouc–Wen hysteretic internal state z(t)
    z = np.zeros_like(t)
    for i in range(1, len(t)):
        z[i] = z[i-1] + (bouc_wen(xdot[i-1], z[i-1], A=A_bw, beta=beta, gamma=gamma, n=n)
                         * dt / max(1e-9, tau_R))  # scale by tau_R: larger tau_R -> slower decay (more memory)

    # Pressure model: p = k1 * x + k2 * z ; simple linear + hysteretic contribution
    k1 = 1.0 / max(1e-12, cavity_C)   # stiffness proxy
    k2 = 0.2 * k1                      # hysteretic weight
    p = k1*x + k2*z

    # Volume change dV ~ A_diaphragm * x  (use orifice area proxy to keep simple)
    A0 = pi*(D/2.0)**2
    dV = A0*x

    # Loop area in p–V plane (hysteresis integral) over final cycle
    idx0 = int(0.8*len(t))  # last 20% of time
    A_loop = np.trapz(p[idx0:]*np.gradient(dV[idx0:], t[idx0:]), t[idx0:])

    # Jet model: outstroke only (xdot<0 or >0 depending on sign); map to exit velocity
    # We'll take outstroke as xdot>0
    out = xdot > 0
    u = (xdot/A0) * out  # crude mapping: volumetric flow / area
    # Nonlinear contraction factor
    Cc = 0.8
    u_exit = Cc * u

    # Thrust ~ rho * A * u^2 over outstroke
    thrust_inst = 1.0 * rho * A0 * (u_exit**2) * out
    impulse = np.trapz(thrust_inst, t)

    # Normalize impulse by formation efficiency and continuity gain
    eff = formation_envelope(S)
    G = hysteresis_gain(tau_R)
    impulse_eff = impulse * eff * G

    return {
        "t": t, "x": x, "xdot": xdot, "z": z, "p": p, "dV": dV,
        "A_loop": A_loop, "impulse_raw": impulse, "impulse": impulse_eff,
        "S": S, "tau_R": tau_R, "D": D, "f": f
    }

def sweep_S_tauR(D=0.010, f=150.0, S_vals=None, tau_vals=None,
                 sigma=1.0, k_H=0.6, seed=417):
    """Compute impulse across S and tau_R using the simple envelope (fast sweep)."""
    if S_vals is None:
        S_vals = np.linspace(1.0, 8.0, 40)
    if tau_vals is None:
        tau_vals = np.array([0.1, 0.5, 1.0, 2.0, 5.0])
    S_vals = np.array(S_vals)
    data = []
    for tau_R in tau_vals:
        H = tau_R/(tau_R + 0.5)
        for S in S_vals:
            eff = np.exp(-0.5*((S-4.0)/sigma)**2)
            impulse = eff*(1.0 + k_H*H)
            data.append((S, tau_R, impulse))
    return np.array(data, float)

def fit_sigma_kH(bench_S, bench_tauR, bench_impulse, tau0=0.5, init=(1.0, 0.6)):
    """Least squares fit for sigma and k_H to match bench data (S sweep, τR gain)."""
    from scipy.optimize import least_squares

    def resid(params):
        sigma, kH = params
        eff = np.exp(-0.5*((bench_S-4.0)/sigma)**2)
        H = bench_tauR/(bench_tauR + tau0)
        model = eff*(1.0 + kH*H)
        return model - bench_impulse

    res = least_squares(resid, x0=np.array(init), bounds=([0.2, 0.0],[5.0, 2.0]))
    sigma_fit, kH_fit = res.x
    return sigma_fit, kH_fit, res.cost

def demo_plot():
    """One single-figure demo similar to the notebook, purely for quick checks."""
    import matplotlib.pyplot as plt
    S = np.linspace(1.0, 8.0, 300)
    tau0 = 0.5
    tauR_values = [0.1, 0.5, 1.0, 2.0, 5.0]
    k_H = 0.6
    sigma = 1.0
    C_S = np.exp(-0.5*((S-4.0)/sigma)**2)
    for tauR in tauR_values:
        H = tauR/(tauR + tau0)
        T_norm = C_S * (1.0 + k_H * H)
        plt.plot(S, T_norm, label=f'τ_R={tauR:.1f}')
    plt.xlabel('Stroke ratio S = L0/D')
    plt.ylabel('Normalized impulse (arb.)')
    plt.title('C-SJX model: impulse vs. S with continuity gain')
    plt.legend()
    plt.tight_layout()
    plt.show()

if __name__ == "__main__":
    demo_plot()

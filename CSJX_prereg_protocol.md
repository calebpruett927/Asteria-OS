
# C-SJX Prereg Protocol v0.1 (fixed seed: 417)

**Goal:** Demonstrate a synthetic-jet thruster whose *useful impulse scales with continuity* (Φ) via the liner return time τ_R, and whose S-sweep peaks near S≈4 with width σ.

## Frozen Spec (mechanical/electrical)

- Orifice diameter **D = 10 mm** (0.010 m). Sharp-edged or 1 mm radius chamfer, plate thickness 1.5 mm.
- Target stroke ratio **S∈[3,5]**, optimum near **S\*≈4**.
  - Slug length L0 = S·D → **L0 ≈ 40 mm** at S=4.
  - For drive **f = 150 Hz** (half-cycle T/2 ≈ 3.33 ms), mean outstroke exit speed target **U0 ≈ 12 m/s**.
- Diaphragm: **voice-coil actuator**, active diameter 38–40 mm, stroke ±1.5 mm, resonance ≥200 Hz.
- Cavity: 40 mm ID, 25 mm depth (≈31 mL), pressure tap to a 0–5 kPa differential sensor.
- **Liner A (viscoelastic):** silicone/PU elastomer, tanδ≈0.2 @150 Hz, thickness 2 mm → mechanical τ_M≈0.1–0.3 s.
- **Liner B (PCM composite):** microencapsulated paraffin C28 (m.p. 28–34 °C) in silicone binder.
  - Thickness **0.15–0.40 mm** → thermal τ_T≈L²/α with α≈1e-7 m²/s → τ_T≈0.2–1.6 s.
  - Add a thin-film heater to bias near melting for τ_R control up to ≈5 s.
- Sensors: hot-wire anemometer (0–20 m/s) at nozzle, 0–1 N load cell thrust stand, microphone for acoustic.
- Drive: function generator + current amp. Frequency 80–250 Hz. Adjust amplitude to hold S constant while sweeping f.

## Variables & Estimators

- **S (stroke ratio):** from L0/D (compute from commanded waveform).
- **τ_R (return time):** exponential tail fit on pressure decay after step-off.
- **Loop area A_loop:** ∮ p dV over last cycle (pressure from sensor, dV from diaphragm kinematics).
- **Impulse I:** ∫ T(t) dt (thrust stand).
- **Energy_in E:** electrical input per run (V·I integration).

## Acceptance Gates

1. **Loop linearity:** I ∝ A_loop across 10× input doses, R² ≥ 0.95.
2. **Continuity gain:** At fixed S≈4, I increases monotonically with τ_R, spanning **0.1–5 s**.
3. **S envelope:** I(S) peaks near 4 with width parameter σ∈[0.6,1.6].

## Fit & Ledger

- Fit **σ** and **k_H** using `fit_sigma_kH()` against your bench (S_i, τ_Ri, I_i).
- Log **Δκ** per run: Δκ ≈ −a·Energy_in − b·θ_Σ − c·τ_R (start with a=1e−3, b=0, c=1e−2; revise post hoc).
- Export CSV using the provided template; attach plot seeds/checksums.

## Cloning Test (Portability)

- Swap Liner A↔B with no control-law changes.
- Re-fit σ,k_H; Passing if parameters shift <20% and gates 1–3 hold.

## Seeds

- Simulation seed: **417**. Use the same for figure regeneration.


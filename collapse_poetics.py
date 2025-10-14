class CollapsePoeticsInterpreter:
    def __init__(self, phrase):
        self.phrase = phrase.lower()

    def interpret(self):
        interpretations = []
        if "doorway" in self.phrase:
            interpretations.append("Doorway → anchor point in κ-space")
        if "edge of a room" in self.phrase:
            interpretations.append("Edge of room → not a boundary, but a reference rejection")
        if "act of stepping" in self.phrase:
            interpretations.append("Act of stepping → drift initiation (ω > 0)")
        if "threshold" in self.phrase:
            interpretations.append("Threshold → curvature fold, not scalar crossing")
        if "fold in time" in self.phrase:
            interpretations.append("Fold in time → τR > 0 (return delay)")
        if "mirror" in self.phrase:
            interpretations.append("Mirror → κ-preserving reflection")
        if "spiral" in self.phrase:
            interpretations.append("Spiral → C > 0 (high curvature path)")
        if "wall" in self.phrase:
            interpretations.append("Wall → ω → 1⁻ (saturation boundary)")
        return interpretations

def generate_poetics_from_invariants(omega=None, C=None, tau_R=None, kappa=None):
    lines = []
    if omega is not None:
        lines.append("The ground slipped — a drift unbound." if omega > 0.3 else "It moved, but softly — a whisper of change.")
    if C is not None:
        lines.append("A spiral deepened — pattern without peace." if C > 0.2 else "It curled gently, holding form.")
    if tau_R is not None:
        lines.append("No way back was found — time folded." if tau_R > 5 else "The loop closed — return was near.")
    if kappa is not None:
        lines.append("Integrity broke — the weld failed." if kappa < 0 else "It held. It lived as one.")
    return lines

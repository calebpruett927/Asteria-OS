from flask import Flask, request, jsonify
from flask_cors import CORS
from collapse_poetics import CollapsePoeticsInterpreter, generate_poetics_from_invariants

app = Flask(__name__)
CORS(app)

@app.route('/interpret', methods=['POST'])
def interpret():
    phrase = request.json.get('phrase', '')
    interpreter = CollapsePoeticsInterpreter(phrase)
    return jsonify({"interpretation": interpreter.interpret()})

@app.route('/generate', methods=['POST'])
def generate():
    data = request.json
    omega = float(data.get('omega', 0))
    curvature = float(data.get('curvature', 0))
    tau_R = float(data.get('tauR', 0))
    kappa = float(data.get('kappa', 0))
    result = generate_poetics_from_invariants(omega, curvature, tau_R, kappa)
    return jsonify({"poetic_fragment": result})

if __name__ == '__main__':
    app.run(debug=True)

import { useState } from 'react';
import { Card, CardContent } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';

export default function CollapsePoeticsUI() {
  const [phrase, setPhrase] = useState('');
  const [invariants, setInvariants] = useState({ omega: '', curvature: '', tauR: '', kappa: '' });
  const [output, setOutput] = useState('');

  const interpretPoetics = async () => {
    const response = await fetch('http://localhost:5000/interpret', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ phrase })
    });
    const data = await response.json();
    setOutput(data.interpretation.join('\n'));
  };

  const generatePoetics = async () => {
    const response = await fetch('http://localhost:5000/generate', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(invariants)
    });
    const data = await response.json();
    setOutput(data.poetic_fragment.join('\n'));
  };

  return (
    <div className="p-4 max-w-2xl mx-auto space-y-4">
      <Card>
        <CardContent className="space-y-2">
          <h2 className="text-xl font-semibold">Collapse Poetics Interpreter</h2>
          <Input placeholder="Enter poetic phrase..." value={phrase} onChange={(e) => setPhrase(e.target.value)} />
          <Button onClick={interpretPoetics}>Interpret Phrase</Button>
        </CardContent>
      </Card>

      <Card>
        <CardContent className="space-y-2">
          <h2 className="text-xl font-semibold">Generate Poetics from Invariants</h2>
          <Input placeholder="Drift ω" value={invariants.omega} onChange={(e) => setInvariants({ ...invariants, omega: e.target.value })} />
          <Input placeholder="Curvature C" value={invariants.curvature} onChange={(e) => setInvariants({ ...invariants, curvature: e.target.value })} />
          <Input placeholder="Return Delay τR" value={invariants.tauR} onChange={(e) => setInvariants({ ...invariants, tauR: e.target.value })} />
          <Input placeholder="Log-Integrity κ" value={invariants.kappa} onChange={(e) => setInvariants({ ...invariants, kappa: e.target.value })} />
          <Button onClick={generatePoetics}>Generate Fragment</Button>
        </CardContent>
      </Card>

      {output && (
        <Card>
          <CardContent>
            <h3 className="font-semibold">Output</h3>
            <pre className="whitespace-pre-wrap text-sm mt-2">{output}</pre>
          </CardContent>
        </Card>
      )}
    </div>
  );
}

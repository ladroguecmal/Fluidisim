"""Contre-épreuves synthétiques de l'instrument S269, sans solveur ni dépendance."""
from contextlib import redirect_stdout
import io
from pathlib import Path
import tempfile
import unittest
import reflexion_paquet as measure

class ReflectionInstrument(unittest.TestCase):
    def prepare(self, root, bad_guard=False):
        for res in ['025', '0125']:
            for name in ['garde','garde-longue','garde-eponge','garde-longue-eponge','mur','eponge']:
                late = 0.02 + {'mur':1.,'eponge':0.005}.get(name,0.)
                if bad_guard and name == 'garde-longue-eponge': late += 0.002
                rows = ['MESURE cas=synthetique']
                for t in range(5,36001,5):
                    y = 1. if t == 4000 else late if t == 22000 else 0.
                    rows.append(f'TRACE t={t/1000:.3f} eta={y:.10f}')
                (root/f'fluidisim-s269-{name}-{res}.log').write_text('\n'.join(rows),encoding='utf-8')

    def test_common_tail_is_removed_but_boundary_response_is_kept(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory); self.prepare(root)
            output=io.StringIO()
            with redirect_stdout(output): measure.compare(root)
            self.assertIn('retour=0.005',output.getvalue())
            self.assertIn('RECU :',output.getvalue())

    def test_contaminated_second_guard_refuses(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory); self.prepare(root,True)
            with redirect_stdout(io.StringIO()), self.assertRaisesRegex(ValueError,'garde différentielle contaminée'):
                measure.compare(root)

    def test_missing_samples_refuse(self):
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory)
            (root/'fluidisim-s269-garde-025.log').write_text('MESURE cas=x\nTRACE t=0.005 eta=0.0',encoding='utf-8')
            with self.assertRaisesRegex(ValueError,'trace incomplète'):
                measure.read(root,'garde','025')

if __name__ == '__main__': unittest.main()

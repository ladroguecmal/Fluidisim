"""S297 : habillage et animation des PPM de banc, sans lire ni écrire d'état de simulation."""
from pathlib import Path
import argparse
from PIL import Image, ImageDraw, ImageFont

parser=argparse.ArgumentParser()
parser.add_argument('directory',nargs='?',default='viewer/captures/s297')
parser.add_argument('--spectral',action='store_true',help='Habillage de la fixture spectrale S298')
args=parser.parse_args()
root=Path(args.directory)
fontroot=Path('C:/Windows/Fonts')
def font(size,bold=False):
    return ImageFont.truetype(str(fontroot/('segoeuib.ttf' if bold else 'segoeui.ttf')),size)
frames=[]
files=sorted(root.glob('frame_*.ppm'))
if not files: raise SystemExit('Aucune image PPM de banc')
for n,path in enumerate(files):
    im=Image.open(path).convert('RGB')
    d=ImageDraw.Draw(im)
    d.text((44,26),('EAU 3D  /  FOND SPECTRAL DU CŒUR' if args.spectral else 'EAU 3D  /  PREMIER APERÇU DU COUPLAGE'),font=font(25,True),fill='#e9f2f5')
    d.text((44,67),('Une impulsion locale dans 64 composantes de vagues directionnelles' if args.spectral else 'Une impulsion locale dans deux ondes de fond croisées'),font=font(18),fill='#a5bac7')
    d.line((44,108,1156,108),fill='#2d4655',width=1)
    d.text((44,128),'01   Surface totale',font=font(21,True),fill='#66d4e6')
    d.text((44,161),'Hauteurs à l’échelle réelle • fond + perturbation',font=font(15),fill='#b5c4cc')
    d.text((644,128),'02   Effet de l’impulsion',font=font(21,True),fill='#efce80')
    d.text((644,161),'Écart au témoin sans impulsion • relief amplifié ×4',font=font(15),fill='#b5c4cc')
    d.text((44,529),f'Domaine : 8 × 6 m  |  Profondeur : {8 if args.spectral else 2} m  |  Maille : 25 cm',font=font(16),fill='#b5c4cc')
    d.text((44,560),'Calcul physique CPU • aperçu de banc • rendu de jeu et GPU à venir',font=font(16),fill='#819aa9')
    t=n*0.05
    d.text((1024,532),f'{t:04.2f} s',font=font(24,True),fill='#e9f2f5')
    d.rounded_rectangle((44,607,1156,611),radius=2,fill='#2b4351')
    d.rounded_rectangle((44,607,44+max(2,1112*n/max(1,len(files)-1)),611),radius=2,fill='#61cadb')
    frames.append(im)
    if n in [0,30,60,120]:im.save(root/f'apercu_{n:04}.png')
# Palette commune : évite les changements de couleurs entre images.
reference=Image.new('RGB',(1200,650*4))
for i,n in enumerate([0,len(frames)//3,2*len(frames)//3,len(frames)-1]):reference.paste(frames[n],(0,i*650))
palette=reference.quantize(colors=192,method=Image.Quantize.MEDIANCUT)
quantized=[im.quantize(palette=palette,dither=Image.Dither.NONE) for im in frames]
quantized[0].save(root/'couplage_3d.gif',save_all=True,append_images=quantized[1:],duration=50,loop=0,optimize=False,disposal=2)
print(f'{len(frames)} images, {len(frames)*0.05:.2f} secondes, {root / "couplage_3d.gif"}')
"""Une vidéo peut être produite en option si imageio_ffmpeg est disponible."""
try:
    import imageio_ffmpeg
    writer=imageio_ffmpeg.write_frames(str(root/'couplage_3d.mp4'),(1200,650),fps=20,codec='libx264',pix_fmt_in='rgb24',pix_fmt_out='yuv420p',macro_block_size=2,quality=8)
    writer.send(None)
    for im in frames:writer.send(im.tobytes())
    writer.close()
    print(root/'couplage_3d.mp4')
except ImportError:
    pass

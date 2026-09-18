//! S248 — **images locales de banc** : la topologie de la mer, et le maillage du LOD tel qu'il
//! tombe sur l'eau. [ADR-124](../../docs/adr/ADR-124-image-budget-et-effets-bornes.md) l'autorise
//! explicitement depuis S201 : **PPM local, aucune publication, aucune page**.
//!
//! Ces images ne sont pas des illustrations. S247 a mesuré que l'écart entre deux sommets voisins
//! atteint 8,243 m à incidence rasante contre 2,589 m à la pose de référence, quand `λ_min` vaut
//! 2,094 m ; ce sont des nombres, et personne n'a vu **où** ils tombent. Chaque image porte donc son
//! empreinte FNV, comme toute image de banc depuis S201, et les extrema qu'elle affiche doivent
//! retrouver ceux que S247 a publiés — sans quoi la carte est fausse, et on le sait avant de la
//! regarder.
use std::fs;
use std::io::{self, BufWriter, Write};

/// Répertoire des images de cette session. Local, jamais publié.
pub const DIR: &str = captures!("s248");

/// Écrit une image PPM binaire et rend son **empreinte FNV-1a**, convention des images de banc
/// depuis S201. Deux exécutions doivent rendre la même (I-03).
pub fn write_ppm(name: &str, width: usize, height: usize, rgb: &[u8]) -> io::Result<u64> {
    if rgb.len() != width * height * 3 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "dimensions RGB"));
    }
    fs::create_dir_all(DIR)?;
    let path = format!("{DIR}/{name}");
    let mut file = BufWriter::new(fs::File::create(&path)?);
    write!(file, "P6\n{width} {height}\n255\n")?;
    file.write_all(rgb)?;
    file.flush()?;
    Ok(rgb
        .iter()
        .fold(0xcbf2_9ce4_8422_2325u64, |h, b| (h ^ *b as u64).wrapping_mul(0x100_0000_01b3)))
}

fn lerp(a: [u8; 3], b: [u8; 3], t: f32) -> [u8; 3] {
    let t = t.clamp(0., 1.);
    [0, 1, 2].map(|i| (a[i] as f32 + (b[i] as f32 - a[i] as f32) * t).round().clamp(0., 255.) as u8)
}

/// Rampe **signée** d'une hauteur d'eau : creux sombres, niveau moyen bleu clair, crêtes blanches.
/// `scale` est l'amplitude qui sature la rampe ; elle est imprimée avec l'image, jamais devinée.
pub fn ramp_height(value: f32, scale: f32) -> [u8; 3] {
    let t = (value / scale.max(1e-6)).clamp(-1., 1.);
    if t < 0. {
        lerp([120, 170, 210], [8, 34, 92], -t)
    } else {
        lerp([120, 170, 210], [250, 250, 255], t)
    }
}

/// Rampe d'**écart entre sommets**, avec une **rupture franche au seuil de Nyquist** : c'est elle
/// qui se lit. Sous le seuil, des verts qui s'éclaircissent — l'échantillonnage suffit. Au-dessus,
/// jaune puis rouge — chaque pixel rouge est une onde que l'image ne peut pas porter.
pub fn ramp_spacing(metres: f32, nyquist: f32, worst: f32) -> [u8; 3] {
    if metres <= nyquist {
        lerp([16, 78, 48], [150, 214, 128], metres / nyquist.max(1e-6))
    } else {
        let span = (worst - nyquist).max(1e-6);
        lerp([255, 222, 92], [190, 24, 18], (metres - nyquist) / span)
    }
}

/// Gris de fond, pour ce qui n'appartient ni à l'eau ni à l'emprise.
pub const HORS: [u8; 3] = [26, 26, 30];

// banc_visuel.js — S471, ADR-216 : **le banc visuel, côté références**. Mesure une vidéo de référence dans la page qui la lit
// (le navigateur décode l'image ; une vidéo servie par blocs — YouTube — se lit dans un canevas sans le salir) ; seuls des nombres
// sortent. Le même calcul, pour nos rendus : `banc_visuel.py` — toute modification se porte dans les deux, et `banc_visuel.py
// --egalite` les confronte (critère : 1 %).
//
// Les grandeurs (par zone, un rectangle en fractions du cadre) reprennent `cible_image.py` (S308) — **comparables sans connaître la
// prise de vue** parce que rapportées à la médiane de la zone — et y ajoutent le temps :
//
//   statiques   p05, p25, p75, p95, p99 sur p50 (luminance linéaire) ; contraste local 9×9 sur p50 ; B/G et B/R des creux (≤ p25) et
//               des crêtes (≥ p75) ; fraction claire (> 4·p50) ; écume (> 2·p50 et saturation < 0,2) ; part haute fréquence
//               (RMS des différences à un pixel ÷ écart-type) ; anisotropie (RMS dy ÷ RMS dx).
//   temporelles mouvement : médiane de |L_t − L_{t−1}| moyen, ÷ p50 et ÷ Δt (par seconde) ; période dominante de la luminance
//               moyenne de la zone (et sa netteté : pic ÷ médiane du spectre) ; renouvellement des clairs : la part des pixels clairs
//               de l'image t qui ne l'étaient pas en t − 1, par seconde (l'inverse d'une durée de vie des reflets).
//
// L'image est d'abord réduite par moyenne de blocs `f × f` des valeurs **linéaires** (`f` entier), dans les deux langages.
//
// La forme est serrée : ce fichier se colle tel quel dans la page de la vidéo (la politique de sécurité de YouTube refuse de le
// charger d'ailleurs) ; une page de YouTube recharge à chaque vidéo, on le garde dans `localStorage.banc_visuel` le temps des mesures.
//
// Usage (dans la page de la vidéo) : coller ce fichier, puis
//   BancVisuel.lancer(document.querySelector('video'), {zones: {mer: [0, 0.1, 1, 0.5]}, fps: 10, debut: 0, fin: 20, facteur: 2})
// et relire `window.__banc` (l'état, puis le résultat) — la mesure est longue, elle ne bloque pas l'appel.

var BancVisuel = (() => {
    const LIN = new Float64Array(256);
    for (let v = 0; v < 256; v++) { const c = v / 255; LIN[v] = c <= 0.04045 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4); }
    const LUM = [0.2126, 0.7152, 0.0722];
    // RGBA (octets) `w × h` → r, g, b, l linéaires réduits par blocs `f × f`.
    function reduire(data, w, h, f) {
        const W = Math.floor(w / f), H = Math.floor(h / f), n = W * H;
        const r = new Float64Array(n), g = new Float64Array(n), b = new Float64Array(n), l = new Float64Array(n);
        for (let J = 0; J < H; J++) for (let I = 0; I < W; I++) {
            let sr = 0, sg = 0, sb = 0;
            for (let dj = 0; dj < f; dj++) { let k = 4 * ((J * f + dj) * w + I * f); for (let di = 0; di < f; di++, k += 4) { sr += LIN[data[k]]; sg += LIN[data[k + 1]]; sb += LIN[data[k + 2]]; } }
            const q = J * W + I, m = 1 / (f * f); r[q] = sr * m; g[q] = sg * m; b[q] = sb * m; l[q] = LUM[0] * r[q] + LUM[1] * g[q] + LUM[2] * b[q];
        }
        return {w: W, h: H, r, g, b, l};
    }
    // La sous-image d'une zone `[x0, y0, x1, y1]` (fractions).
    function decouper(img, z) {
        const x0 = Math.floor(z[0] * img.w), y0 = Math.floor(z[1] * img.h), x1 = Math.floor(z[2] * img.w), y1 = Math.floor(z[3] * img.h);
        const w = x1 - x0, h = y1 - y0, n = w * h;
        const o = {w, h, r: new Float64Array(n), g: new Float64Array(n), b: new Float64Array(n), l: new Float64Array(n)};
        for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) { const s = (y0 + j) * img.w + x0 + i, d = j * w + i; o.r[d] = img.r[s]; o.g[d] = img.g[s]; o.b[d] = img.b[s]; o.l[d] = img.l[s]; }
        return o;
    }
    function centile(tri, p) { const n = tri.length; return tri[Math.min(n - 1, Math.max(0, Math.floor(p / 100 * (n - 1))))]; }
    function statiques(z) {
        const n = z.l.length, tri = Float64Array.from(z.l).sort(), c = {};
        for (const p of [5, 25, 50, 75, 95, 99]) c[p] = centile(tri, p);
        const med = Math.max(c[50], 1e-9);
        let moy = 0; for (let k = 0; k < n; k++) moy += z.l[k]; moy /= n;
        let v = 0; for (let k = 0; k < n; k++) v += (z.l[k] - moy) ** 2;
        // Le contraste local : l'écart-type dans une fenêtre 9×9, tous les 7 pixels ; la médiane.
        const et = Math.sqrt(v / n), locaux = [];
        for (let j = 4; j < z.h - 4; j += 7) for (let i = 4; i < z.w - 4; i += 7) {
            let s = 0, s2 = 0;
            for (let dj = -4; dj <= 4; dj++) for (let di = -4; di <= 4; di++) { const x = z.l[(j + dj) * z.w + i + di]; s += x; s2 += x * x; }
            const m = s / 81; locaux.push(Math.sqrt(Math.max(s2 / 81 - m * m, 0)));
        }
        locaux.sort((a, b) => a - b);
        const contraste = locaux.length ? locaux[Math.floor(locaux.length / 2)] : 0;
        const teinte = (bas) => { let sr = 0, sg = 0, sb = 0; for (let k = 0; k < n; k++) if (bas ? z.l[k] <= c[25] : z.l[k] >= c[75]) { sr += z.r[k]; sg += z.g[k]; sb += z.b[k]; } return [sb / Math.max(sg, 1e-12), sb / Math.max(sr, 1e-12)]; };
        const creux = teinte(true), cretes = teinte(false);
        let claire = 0, ecume = 0;
        for (let k = 0; k < n; k++) { if (z.l[k] > 4 * med) claire++; const mx = Math.max(z.r[k], z.g[k], z.b[k]), mn = Math.min(z.r[k], z.g[k], z.b[k]); if (z.l[k] > 2 * med && (mx - mn) < 0.2 * Math.max(mx, 1e-12)) ecume++; }
        let sx = 0, nx = 0, sy = 0, ny = 0;
        for (let j = 0; j < z.h; j++) for (let i = 0; i < z.w; i++) { const k = j * z.w + i; if (i + 1 < z.w) { sx += (z.l[k + 1] - z.l[k]) ** 2; nx++; } if (j + 1 < z.h) { sy += (z.l[k + z.w] - z.l[k]) ** 2; ny++; } }
        const rx = Math.sqrt(sx / Math.max(nx, 1)), ry = Math.sqrt(sy / Math.max(ny, 1));
        return {p05: c[5] / med, p25: c[25] / med, p75: c[75] / med, p95: c[95] / med, p99: c[99] / med, contraste: contraste / med,
            creux_BsurG: creux[0], creux_BsurR: creux[1], cretes_BsurG: cretes[0], cretes_BsurR: cretes[1], claire: claire / n, ecume: ecume / n,
            hf_part: Math.sqrt((rx * rx + ry * ry) / 2) / Math.max(et, 1e-12), anisotropie: ry / Math.max(rx, 1e-12), p50_absolu: med};
    }
    function mediane(a) { const t = Float64Array.from(a).sort(); return t.length ? t[Math.floor(t.length / 2)] : NaN; }
    // Les grandeurs temporelles d'une suite de zones (même découpe), espacées de `dt` s.
    function temporelles(suite, dt) {
        const N = suite.length; if (N < 3) return {};
        const meds = suite.map(z => mediane(z.l)), med = mediane(meds), dif = [], renouv = [], moy = [];
        for (let t = 0; t < N; t++) {
            const z = suite[t]; let s = 0; for (let k = 0; k < z.l.length; k++) s += z.l[k]; moy.push(s / z.l.length);
            if (t === 0) continue;
            const a = suite[t - 1]; let d = 0, clairs = 0, neufs = 0;
            for (let k = 0; k < z.l.length; k++) { d += Math.abs(z.l[k] - a.l[k]); if (z.l[k] > 4 * meds[t]) { clairs++; if (!(a.l[k] > 4 * meds[t - 1])) neufs++; } }
            dif.push(d / z.l.length); renouv.push(clairs > 0 ? neufs / clairs : NaN);
        }
        // Les coupes d'un montage : un écart de plus de six fois la médiane, et de 5 % de la luminance au moins (le bruit d'une zone
        // immobile n'en est pas une) ; exclues, et le spectre pris sur le plus long plan.
        const seuil = Math.max(6 * mediane(dif), 0.05 * med), coupe = dif.map(d => d > seuil);
        const difs = dif.filter((d, t) => !coupe[t]), renouvs = renouv.filter((r, t) => !coupe[t] && !Number.isNaN(r));
        let plan = [0, 0], deb = 0;
        for (let t = 1; t <= N; t++) if (t === N || coupe[t - 1]) { if (t - deb > plan[1] - plan[0]) plan = [deb, t]; deb = t; }
        const moyp = moy.slice(plan[0], plan[1]), coupes = coupe.filter(c => c).length;
        if (moyp.length < 8) return {coupes, images: N, dt};
        const res = spectre_de(moyp, dt);
        return {mouvement_par_s: mediane(difs) / Math.max(med, 1e-12) / dt, periode_s: res[0], periode_nettete: res[1],
            renouvellement_clairs_par_s: renouvs.length ? mediane(renouvs) / dt : NaN, coupes, plan_s: moyp.length * dt, images: N, dt};
    }
    // Le spectre d'une suite (tendance linéaire retirée), de 1/(durée/2) à Nyquist : la période du pic, sa netteté.
    function spectre_de(moy, dt) {
        const N = moy.length, T = N * dt; let sx = 0, sy = 0, sxx = 0, sxy = 0;
        for (let t = 0; t < N; t++) { sx += t; sy += moy[t]; sxx += t * t; sxy += t * moy[t]; }
        const pente = (N * sxy - sx * sy) / Math.max(N * sxx - sx * sx, 1e-12), orig = (sy - pente * sx) / N;
        const x = moy.map((m, t) => m - (orig + pente * t)), spectre = [];
        for (let k = 2; k <= Math.floor(N / 2); k++) { let re = 0, im = 0; for (let t = 0; t < N; t++) { const a = 2 * Math.PI * k * t / N; re += x[t] * Math.cos(a); im -= x[t] * Math.sin(a); } spectre.push([k / T, re * re + im * im]); }
        let pic = [NaN, 0]; for (const s of spectre) if (s[1] > pic[1]) pic = s;
        return [1 / pic[0], pic[1] / Math.max(mediane(spectre.map(s => s[1])), 1e-30)];
    }
    function mesurer_suite(images, zones, dt) {
        const res = {};
        for (const [nom, z] of Object.entries(zones)) { const suite = images.map(img => decouper(img, z)), st = suite.map(statiques), m = {}; for (const k of Object.keys(st[0])) m[k] = mediane(st.map(s => s[k])); res[nom] = {statiques: m, temporelles: temporelles(suite, dt)}; }
        return res;
    }
    const arrondir = (o) => JSON.parse(JSON.stringify(o, (k, v) => typeof v === 'number' ? Number(v.toPrecision(4)) : v));
    // Une vidéo : `fps` images par seconde de `debut` à `fin` (s), réduites par `facteur`.
    async function lancer(video, opt) {
        const fps = opt.fps || 10, facteur = opt.facteur || 2, fin = Math.min(opt.fin || video.duration, video.duration - 0.05), debut = opt.debut || 0;
        video.pause();
        const c = document.createElement('canvas'); c.width = video.videoWidth; c.height = video.videoHeight;
        const g = c.getContext('2d', {willReadFrequently: true}), images = [];
        window.__banc = {etat: 'en cours', faites: 0};
        for (let t = debut; t <= fin; t += 1 / fps) {
            await new Promise(ok => { video.addEventListener('seeked', ok, {once: true}); video.currentTime = t; });
            g.drawImage(video, 0, 0); images.push(reduire(g.getImageData(0, 0, c.width, c.height).data, c.width, c.height, facteur)); window.__banc.faites = images.length;
        }
        window.__banc = {etat: 'fini', largeur: c.width, hauteur: c.height, facteur, fps, debut, fin, duree_video: video.duration, zones: opt.zones, mesures: arrondir(mesurer_suite(images, opt.zones, 1 / fps))};
        return window.__banc;
    }
    // Des images (des URL de même origine), espacées de `dt` s — l'égalité avec `banc_visuel.py`.
    async function images(urls, zones, dt, facteur) {
        const imgs = [];
        for (const u of urls) { const im = new Image(); await new Promise((ok, ko) => { im.onload = ok; im.onerror = ko; im.src = u; }); const c = document.createElement('canvas'); c.width = im.naturalWidth; c.height = im.naturalHeight; const g = c.getContext('2d', {willReadFrequently: true}); g.drawImage(im, 0, 0); imgs.push(reduire(g.getImageData(0, 0, c.width, c.height).data, c.width, c.height, facteur)); }
        return arrondir(mesurer_suite(imgs, zones, dt));
    }
    return {lancer, images};
})();

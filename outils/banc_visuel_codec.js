// banc_visuel_codec.js — S473, ADR-216 : **comparer à traitement égal**. Une vidéo de référence est compressée (YouTube : AV1 ou
// VP9, quelques centaines de kbit/s) ; la compression lisse le scintillement fin de l'eau — le mouvement mesuré d'une séquence de
// Godot tombe de 1,29 à 0,45 par seconde en H.264 à 1 Mbit/s (S473). Avant de comparer nos grandeurs fines et temporelles à une
// référence compressée, notre séquence passe donc par le même codec, au même débit, dans le navigateur (WebCodecs : encodé puis
// décodé, rien n'est téléchargé), puis par `BancVisuel.images` (`banc_visuel.js`, à charger avant ce fichier).
//
// Usage (page servie depuis la racine du dépôt, `outils/banc_visuel_codec.html`) :
//   BancCodec.lancer({motif: '/godot/captures/miroir/plage_cote_', n: 270, chiffres: 3, largeur: 480, hauteur: 854, fps: 30,
//                     codec: 'av01.0.04M.08', debit: 590000, pas: 2, zones: {...}, facteur: 2})
// puis relire `window.__codec` (`etat`, puis `brut` et `comp` : les mesures sans et avec le codec). `pas` : une image sur `pas`
// est mesurée (la cadence de mesure de la référence). Le codec et le débit d'une référence YouTube se lisent dans sa page :
// `document.getElementById('shorts-player').getStatsForNerds()` (`codecs`, `resolution`) et les octets reçus.

var BancCodec = (() => {
    async function compresser(o) {
        const urls = [];
        for (let k = 0; k < o.n; k++) urls.push(o.motif + String(k).padStart(o.chiffres, '0') + '.png');
        const morceaux = [];
        let config = null;
        const enc = new VideoEncoder({
            output: (c, meta) => { morceaux.push(c); if (meta && meta.decoderConfig) config = meta.decoderConfig; },
            error: (e) => { throw e; },
        });
        enc.configure({codec: o.codec, width: o.largeur, height: o.hauteur, bitrate: o.debit, framerate: o.fps});
        for (let k = 0; k < o.n; k++) {
            const im = new Image();
            await new Promise((ok, ko) => { im.onload = ok; im.onerror = ko; im.src = urls[k]; });
            const bmp = await createImageBitmap(im);
            const vf = new VideoFrame(bmp, {timestamp: Math.round(k * 1e6 / o.fps)});
            // Une image clé toutes les cinq secondes, comme un flux de diffusion.
            enc.encode(vf, {keyFrame: k % (5 * o.fps) === 0});
            vf.close();
            bmp.close();
            if (enc.encodeQueueSize > 8) await new Promise(r => setTimeout(r, 5));
        }
        await enc.flush();
        const octets = morceaux.reduce((s, c) => s + c.byteLength, 0);
        // Le décodeur garde un nombre fini d'images : chacune est copiée puis fermée dans son rappel.
        const promesses = [];
        let n = 0;
        const dec = new VideoDecoder({
            output: (f) => {
                const k = n++;
                if (k % o.pas === 0) {
                    const oc = new OffscreenCanvas(o.largeur, o.hauteur);
                    oc.getContext('2d').drawImage(f, 0, 0);
                    promesses[k / o.pas] = oc.convertToBlob({type: 'image/png'}).then(b => URL.createObjectURL(b));
                }
                f.close();
            },
            error: (e) => { throw e; },
        });
        dec.configure(config);
        for (const c of morceaux) dec.decode(c);
        await dec.flush();
        const comp = await Promise.all(promesses);
        return {urls: urls.filter((u, k) => k % o.pas === 0), comp, kbps: octets * 8 / (o.n / o.fps) / 1000};
    }

    async function lancer(o) {
        window.__codec = {etat: 'en cours'};
        try {
            const r = await compresser(o);
            const dt = o.pas / o.fps;
            const brut = await BancVisuel.images(r.urls, o.zones, dt, o.facteur);
            const comp = await BancVisuel.images(r.comp, o.zones, dt, o.facteur);
            window.__codec = {etat: 'fini', codec: o.codec, debit_demande: o.debit, debit_obtenu_kbps: Math.round(r.kbps), brut, comp};
        } catch (e) {
            window.__codec = {etat: 'erreur ' + e};
        }
        return window.__codec;
    }

    return {lancer};
})();

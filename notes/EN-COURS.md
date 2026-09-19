# Travail en cours — journal d'intention

> **Pourquoi ce fichier existe.** Une session coupée par une limite d'usage n'a *aucune* occasion
> d'écrire « j'ai été interrompue ». Tout dispositif de passation qui suppose une action au moment
> de l'arrêt est donc inutile. Seule survit une déclaration faite **avant** le travail.
>
> Ce fichier déclare ce qui va être fait, avant de le faire. Git enregistre ce qui a effectivement
> été fait. L'écart entre les deux est exactement ce qui a été interrompu.

---

## Reprise à chaud — procédure

À suivre lorsque l'état ci-dessous n'est pas `terminée`. Cinq minutes ; **ne pas lire tout le
dépôt** — la lecture complète (`REPRISE.md`) ne sert qu'au démarrage à froid.

1. **Lire l'état et le plan** de la session en cours, plus bas.
2. `git log --oneline -15` — **ce qui est committé est fait**, définitivement. Ne pas le refaire.
3. `git status --short` — les fichiers modifiés non committés appartiennent à l'étape marquée
   `[>]`. C'est elle qui a été interrompue, et elle seule.
4. `git diff` — **lire avant de décider**. Deux issues, pas trois :
   - **compléter** l'étape, si le diff est cohérent et si la thèse déclarée dans le plan est
     claire ;
   - **annuler** l'étape (`git restore <fichiers>`), si le diff est incohérent ou
     incompréhensible.

   Ne jamais laisser un état intermédiaire non tranché, et écrire dans le journal lequel des deux
   a été choisi.
5. **Lire les notes de reprise** de la session interrompue. C'est là que vivent les chiffres déjà
   calculés, les décisions prises mais pas encore écrites et les impasses déjà explorées —
   l'information la plus coûteuse à reproduire, et la seule que git ne conserve pas.
6. Reprendre au premier `[ ]`, ou à `[>]` si l'étape a été complétée.
7. **Prévenir l'utilisateur** : la session précédente a probablement été coupée avant d'avoir pu
   rendre compte de son travail. Résumer ce qu'elle avait fait — il ne l'a peut-être jamais vu.

---

## Règles pour la session qui travaille

- **Déclarer le plan complet avant la première modification**, et le committer seul. C'est
  l'écriture anticipée : sans elle, une interruption ne laisse aucune trace d'intention.
- **Aucune étape ne dépasse une quinzaine de minutes de travail.** Si elle est plus grosse, la
  découper. C'est la seule prophylaxie réelle contre une coupure — pas un confort d'organisation.
- Marquer `[>]` **avant** de commencer une étape. Basculer `[x]` **en dernière action avant le
  commit de cette étape**, jamais après : le commit doit contenir à la fois le travail et la case
  cochée, sinon l'historique ment dans un sens ou dans l'autre. Un `[x]` sans commit est un
  mensonge que la session suivante paiera ; un commit sans `[x]` fera refaire du travail déjà fait.
- **Un commit par étape**, message `S<n> P<k> — <description>`. Le plan et le journal git disent
  alors la même chose de deux façons indépendantes ; si l'un est faux, l'autre le révèle.
- Déposer dans **Notes de reprise** tout ce qui n'est pas encore dans un fichier : un chiffre
  calculé, une décision prise, une impasse explorée. **Une impasse est aussi précieuse qu'un
  résultat** — sans elle, la session suivante la réexplore intégralement.
- **Le rituel de fin (`REPRISE.md` §6) est lui-même une étape du plan.** Une session interrompue
  laisse ainsi cette étape visiblement non cochée, ce qui dit à la suivante exactement ce qui
  manque.

---

## Session en cours

Session : S292 — **interrompue par l'utilisateur** après P2/P3
Agent : Claude Opus 5, application desktop Claude Code ; fichiers, git, cargo, outils locaux.
Entrée : demande de l'utilisateur — régler le gel de la carte refroidie. C'est A294.
Objectif : que le **pire** pas tienne, y compris après une inactivité de la carte. Un pic de
467 ms annule le gain de médiane de milliers de pas.

### Plan

- [x] **P1** — état réel, jeton et plan seuls.
- [x] **P2** — **où tombe le gel** *(banc construit, mesuré — mais le gel ne s'y reproduit pas)* : décomposer le premier appel après inactivité en ses cinq
  postes déjà instrumentés (empaquetage / encodage / soumission / attente / lecture). Cela
  distingue le pilote, la carte et notre code, et interdit de deviner.
- [x] **P3** — **caractériser** *(courbe faite ; elle ne contient pas le phénomène cherché)* : le coût du premier appel en fonction de la durée d'inactivité
  (0 à plusieurs secondes), et le nombre d'appels avant retour au régime. Publier la courbe,
  pas un seul point.
- [ ] **P4** — **discriminer le mécanisme** par des essais qui s'excluent : un envoi *trivial*
  réchauffe-t-il autant qu'un cycle complet ? le gel dépend-il de la taille des tampons
  (résidence mémoire) ou non (état d'alimentation) ? une simple lecture d'horodatage suffit-elle ?
- [ ] **P5** — **construire l'entretien** que la mesure désigne, au bon endroit : le solveur
  expose le geste, l'hôte décide de la cadence. Mesurer son coût propre et ses allocations.
- [ ] **P6** — **éprouver sur le chemin réel** : pas couplé de la bande δ avec des coupures
  franches, pire pas mesuré avec et sans entretien, trajectoire inchangée au bit.
- [ ] **P7** — rituel §6 : preuve, journal, registres/index/feuille, jeton libre.

### Notes de reprise

Ce que S291 a mesuré, et qui est le point de départ ([preuve](../docs/validation/PAS-DECOMPOSE-S291.md) §7) :
un pas à **467 ms**, puis **132 ms** à la reproduction, toujours au premier appel venant après
≈ 7 s sans GPU. Un envoi de préchauffage **au démarrage** ne le supprime pas. Ne pas laisser la
carte inactive le supprime : maximum retombé à 26,12 ms. Donc **chemin refroidi, pas chemin neuf**.

**Le piège de cette session est de bâtir l'entretien avant de savoir ce qui refroidit.** Trois
mécanismes possibles, et ils ne demandent pas le même geste :
1. **état d'alimentation** de la carte — elle redescend en fréquence, et le premier envoi paie la
   remontée. Un envoi trivial suffirait à l'empêcher, et le coût serait indépendant de la taille.
2. **résidence mémoire** — les tampons sortent de la VRAM et le premier envoi paie leur retour.
   Le coût dépendrait alors de la taille des tampons, et un envoi trivial ne protégerait rien.
3. **caches internes de la pile** (allocateurs de commandes, ceinture de transfert) — le coût
   serait dans l'encodage ou la soumission, pas dans l'attente.
P2 et P4 existent pour trancher entre les trois, et le décompte des cinq postes le dit presque
seul : dans l'**attente**, c'est la carte ou le pilote ; dans la **soumission**, le pilote ;
dans l'**encodage**, wgpu.

### Ce que la session a établi avant d'être arrêtée

**Le gel de 467 ms ne se reproduit sous aucun des deux modèles d'inactivité essayés.** Banc
`--pas-froid` : huit appels dos à dos, puis une pause, puis l'appel froid décomposé en ses cinq
postes, trois répétitions par durée. 6 656 mailles, cycle 128, horodatage éteint dans les deux
bras.

| pause | endormie : froid / chaud | occupée (CPU saturé) : froid / chaud |
|---|---|---|
| 0 ms | 2,2935 / 1,83–2,15 | 3,0606 / 2,22–2,25 |
| 500 ms | 2,7587 / 1,99–2,37 | 2,8324 / 2,01–2,50 |
| 1 s | 3,1367 / 2,20–3,31 | 2,9953 / 1,99–2,31 |
| 2 s | 3,8905 / 2,48–2,72 | 4,3519 / 2,15–3,43 |
| 4 s | 4,8361 / 2,33–3,73 | 4,0555 / 2,16–2,85 |
| 8 s | 4,6458 / 2,51–3,50 | 4,1754 / 2,39–2,86 |

Il y a donc **bien un refroidissement, mais il vaut ×1,4 à ×2, pas ×200**. Il apparaît à partir
de ≈ 2 s de pause et sature ensuite. **Il tombe dans l'attente** — 0,32 → 1,77 ms avec CPU
occupé, 0,32 → 0,78 endormi — donc du côté carte ou pilote, pas dans notre encodage ni dans
notre empaquetage, qui bougent peu. C'est cohérent avec un **état d'alimentation** plutôt qu'avec
une résidence mémoire, mais ce n'est pas tranché : P4 existait pour cela.

**Donc l'hypothèse de S291 est fausse ou incomplète**, et il ne fallait surtout pas bâtir
l'entretien dessus. Ce que S291 a observé (467 puis 132 ms) est un pic du **pas complet** mesuré
dans `--pas-couple`, pas un appel du solveur mesuré isolément. Les différences restantes entre
les deux bancs, à instruire dans cet ordre :
1. `--pas-couple` construit un **`Volume` neuf par régime**, avec une réserve de 64 Mo — le
   premier pas d'un régime paie donc les défauts de page de tous ses tableaux. Cela expliquerait
   un pic au premier pas **de chaque** régime, ce qui n'est pas ce qui a été vu — à vérifier.
2. La pause y était remplie par 400 pas de calcul **couplé**, pas par un ballast synthétique.
3. Le pic mesuré est celui du **pas**, qui contient bien plus que l'appel du solveur.

### Reprise : le geste suivant est déjà écrit et compilé

`--pas-couple` a été modifié pour **garder la carte du pire pas** : étapes du cœur et cinq postes
de l'appel au moment du pic (`PIC_S292`). L'ordre des régimes a été remis à celui de S291, où le
phénomène était apparu. **Cela compile mais n'a jamais été exécuté.** La commande est :

```
cargo run --release --offline --locked --manifest-path viewer/Cargo.toml -- --pas-couple
```

Elle dira en un passage si le pic est dans `candidat` (chemin GPU), dans `sauvegarde` ou
`iterations` (cœur et mémoire), ou ailleurs — et donc lequel des trois points ci-dessus est le
bon. **Ne pas construire d'entretien avant cette réponse.**

Ce qui reste hors de ce lotCe qui reste hors de ce lot : budget 2 ms, longueur de cycle calibrée, multiplateforme, 3D,
solides, boucle d'image complète. Et **aucune porte d'acceptation ne bouge** : ADR-143/144
restent le garde-fou qui rend toute mesure d'identité vérifiable au bit.

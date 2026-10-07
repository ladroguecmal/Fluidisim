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
- **Ce fichier ne porte que la session en cours** ([ADR-187](../docs/adr/ADR-187-methode-refondue-s321.md)
  D3). À la clôture, ce qui doit survivre des notes va à la preuve ou au journal ; la session
  suivante remplace ensuite toute la section. Aucune section d'archive, 300 lignes au plus :
  `outils/etat_projet.py --check` le vérifie. Notes de S301 à S320 : `git show 78622a19:notes/EN-COURS.md`.

---

## Session en cours

Session : S658 — **terminée**. Sur la demande de l'utilisateur (2026-10-07) : *« J'aimerais des sessions visuelles grâce à toutes les
nouvelles avancées »*. **La séance visuelle du rouleau** : le montage complet de S650–S657 — Saint-Venant 2D au large, APIC 3D sur la
plage, le déferlement plongeant, la poche d'air, la sphère libre emportée — enregistré, rendu en animations, envoyé pour un verdict (R40).

**Le rendu** : un rendu d'atelier (numpy et PIL, sans téléchargement), pour juger la physique — la forme du rouleau, le jet, la poche, la
remontée, le corps emporté —, non le rendu final de Godot. Deux animations : la plage entière vue de côté, la zone du rouleau de près ;
les particules colorées par leur vitesse, le fond en escalier, la surface de Saint-Venant au large, la sphère.

**Contrôles du plan** (ADR-266)

- **témoin** : sans objet — la séance montre, elle n'attribue rien.
- **instrument** : l'enregistrement relu avant de rendre — le nombre de particules d'une image égale celui que le calcul compte, la position
  de la sphère celle du relevé de S657 au même instant (à 1 mm).
- **calcul** : les échelles du rendu (m par pixel) calculées par le script, la durée des images assertée (4 s, une image toutes les 0,04 s).
- **ADR** : ADR-216 (le banc visuel ; l'utilisateur juge), ADR-262 (indiscernable du réel : ce rendu ne juge pas le réalisme visuel final).
- **pièges** : la projection de côté superpose toute la largeur (le corps masque l'eau derrière lui) ; une vitesse saturée cache le jet
  (l'échelle des couleurs bornée et dite) ; la taille des fichiers (sous 15 Mo chacun).

**Critères, écrits avant.** (1) L'enregistrement relu (les deux contrôles de l'instrument). (2) Deux animations envoyées à l'utilisateur, avec
ce qu'elles montrent et à quel instant. (3) Le verdict reçu ou attendu, inscrit (R40).

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — l'enregistrement ; le rendu ; l'envoi.
- [ ] **P3** — preuve ; rituel.

### Notes de reprise
- **P2 fini** — (1) relu : 100 images, 46 521 particules, la sphère à 0,048 mm ; (2) deux animations envoyées (la plage, le rouleau) — le
  premier rendu, vu avant l'envoi, laissait vide la zone des colonnes : refait ; (3) R40 en attente (la boussole).

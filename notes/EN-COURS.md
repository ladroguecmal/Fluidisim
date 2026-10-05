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

Session : S485 — **terminée**. En autonomie, **K2-3b — attribuer la remontée lente** de S484 ([REMONTEE-S484](../docs/validation/REMONTEE-S484.md)
§4 : 0,338 m/s contre 0,603 de Davies et Taylor à R/dx = 6).

**Ce que la session fait.** Un cas `CAS=remontee` du banc `--apic3d-poches` : la même cuve que `apic3d_remontee` (quart ou entière),
ensemencée par la référence, menée **par la carte seule** (elle suit la référence à 10⁻⁴, S481–S482, et va cent fois plus vite) ; le
centre de la poche relu à chaque pas, la même droite. Puis les quatre causes de §4, une à une : la résolution (R/dx = 6, 8), le quart
contre la cuve entière, le pas (1 et 2 ms).

**Entrées, et comment elles se vérifient.** Le cas à R/dx = 6 en quart, 1 ms, rejoue d'abord S484 sur la carte : sa vitesse doit
retrouver 0,338 m/s à 5 % — sinon la carte ne mesure pas la même chose, et c'est la première chose à dire.

**Critères, écrits avant.** (1) le rejeu de S484 sur la carte à 5 % ; (2) chaque cause mesurée (U par résolution, par cuve, par pas) ;
(3) U à 15 % de Davies–Taylor dans au moins une configuration résolue — ou la cause nommée, chiffrée, et la suite écrite.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `CAS=remontee` sur la carte ; le rejeu de S484 (1).
- [x] **P3** — résolution, cuve, pas (2), (3).
- [x] **P4** — preuve ; rituel.

### Notes de reprise
- **P2** — `CAS=remontee` : à t = 0,05 s la carte donne la référence de S484 au chiffre (z 0,1182, V 2,4777·10⁻⁴) ; sur 0,4 s elle
  diverge (U = 0,440 contre 0,338, et elle accélère) — **critère (1) manqué** : deux calculs qui coïncident au départ s'écartent.
- **P3** — **trouvé : la bulle perd son air.** (a) L'air d'une poche se partageait entre toutes les nouvelles, fragments d'une à sept
  mailles compris, résorbés aussitôt — corrigé : entre les seules poches gardées (référence et carte). (b) La calotte qui se scinde :
  les particules envahissent la poche haute (160 → 21 mailles en 30 ms, volume suivi inchangé) et elle se résorbe avec son air —
  `RAPPEL_VOLUME_S` 0,1 → **0,02 s** : air conservé à 0,3 % sur 0,6 s. Mesure sur toutes les poches (centre pondéré). **Quart de cuve**
  R/dx = 8 : U = 0,376 (× 0,61), R/dx = 6 : 0,414 (× 0,67), R/dx = 4 : 0,343 (× 0,57) ; **cuve entière R/dx = 4 : U = 0,548 m/s, × 0,90
  de Davies–Taylor** — critère (3) tenu : le quart de cuve était le défaut (les parois d'APIC ne sont pas des plans de symétrie pour une
  bulle qui les longe). La cuve entière à R/dx ≥ 6 dépasse la carte (plus de 65 535 groupes par passe au-delà de ≈ 8 M particules).
  Minnaert (S479, rappel 0,02) : 37,2 Hz — 0,89 de l'eau infinie, à 3 % de la valeur corrigée de la cuve (38,4). Empreintes de
  non-régression réinscrites (le changement voulu ; détecté par le banc : 079c9a99… au lieu de dc06f28c…).
- **P4** — preuve REMONTEE-S485 ; 7.4 ; POCHES-AIR-S479 (B10 à 24 mailles, la grande cuve) ; index ; journal.

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

Session : S755 — **terminée**. En autonomie ; session longue. ADR-289 D3.3, ADR-292 : **refaire le témoin tout-3D de R43 avec la 3D
corrigée.** C'est le montage de S730 E2 (`d` = 0,5 m, `H/d` = 0,3, 1:12, le raccord du rivage à 12,0 m) qui jugeait les raccords et le
sélecteur. **La question** : avec une 3D qui garde l'onde, où et quand la vague de R43 plonge-t-elle, et de combien les mesures changent-elles ?

**L'essai** : `Large::AucunCorrigee` (le tout-3D de S730 E2, 5 s, la projection `Complete` consciente du fond) ; le film à 1/30 s
(`calculs/s755_tout3d_corrigee.bin`), pour un rendu plus tard.

**Les mesures, contre S730 E2 (sans correction)** :
- le retournement (S730 : 2,620 s, 9,938 m) ;
- l'air enfermé (2,804 s ; 10,375 m) ;
- la remontée (0,3547 m à 3,942 s) ;
- le mur au raccord (0 mm) ;
- la masse ;
- le coût (1 152 s).

**Les critères, écrits avant** :
1. la masse à 10⁻¹² ;
2. le mur au raccord sous 1 cm ;
3. un retournement dans le domaine.

Le reste est **rapporté** : c'est le nouveau témoin, sans référence extérieure sur cette plage. SGN (S733) prévoyait le déclenchement plus tard
que l'ancienne 3D (Kennedy 0,65 : 2,73 s ; 10,46 m). La concordance avec SGN est rapportée, non jugée.

**Contrôles du plan** (ADR-285, ADR-286, ADR-289, ADR-292)

- **témoin** : S730 E2 (la 3D sans correction) ; SGN (S733), rapporté.
- **instrument** : ceux de S730 (le juge du retournement de S647, l'air, le mur `J`, la remontée par Saint-Venant au-delà du raccord).
- **calcul** : ≈ 25 à 30 min.
- **ADR** : ADR-292 ; ADR-286 D1 (le raccord à 12,0 m ; S730 n'y voyait rien arriver avant 3,2 s, et la remontée passe dans Saint-Venant) ;
  ADR-285 D1 (le mur mesuré).
- **pièges** : le juge de S647 compte un vide d'une maille à φ ≈ 0 (S735). Le juge robuste (S737) n'est pas branché dans ce montage : un
  retournement très précoce ou loin de la vague serait suspect, et rapporté comme tel.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — le mode ; l'essai ; (1)–(3), les mesures.
- [x] **P3** — preuve ; le lot ; fermeture.

### Notes de reprise
- **P2 fini** (1 235 s) :
  - le retournement à 3,092 s et 11,088 m ; l'air à 3,257 s et 11,425 m ; la remontée 0,3005 m ;
  - le mur 0 ; la masse 1,2·10⁻¹⁶ ;
  - critères tenus.

  La vague déferle 0,47 s plus tard, 1,15 m plus près du rivage. Le raccord à 12,0 m devient trop près (la règle d'ADR-284 : ≈ 12,2 m).

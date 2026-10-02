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

Session : S445 — **terminée**. Demande de l'utilisateur (2026-10-02) : *« Continue »* — la suite déclarée : le lot des registres
(ADR-213 D3), puis c1, sa seconde et dernière session (ADR-213 D2).

**Ce que la session trouve en entrant.** `Apic3` en mode relatif, sous une houle stationnaire partie à plat : `|u′|` croît jusqu'à
54 % de `aω` en 5 s ([preuve](../docs/validation/APIC-CARTE-S416.md) §23.1). Hypothèse : la lecture de la surface des particules,
forcée en résonance avec B.

**Ce que la session fait.** (1) **Le lot des registres** — feuille de route, liste, file active, index, angles morts — pour S443 et S444.
(2) **c1** : départ à plat, l'instrument de S444 (`S444_A`, plus `S444_DX`) ; la loi de `|u′|` en amplitude (1,25, 2,5, 5 cm) et en
maille (25 et 12,5 cm).

**Critères, écrits avant.** **L'hypothèse de la lecture** tient si `|u′|` décroît avec la maille (au moins d'un facteur 1,5 de 25 à
12,5 cm) ; **une faute de couplage** si `|u′|` croît comme `a` et ne dépend pas de la maille ; **une physique du second ordre** (la
cinématique des particules contre la surface linéaire) si `|u′|` croît comme `a²`. **c1 reçu** si un remède ramène `|u′|` sous 2 % de
`aω` sur 5 s (ADR-213 D1 : 5 % de tolérance) ; sinon, **l'eau totale dans la bande** devient la voie (ADR-213 D2), conçue en fin de
session.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le lot des registres.
- [x] **P3** — c1 : les lois ; un remède s'il se montre ; sinon, la conception de l'eau totale dans la bande.
- [x] **P4** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — feuille de route, liste, index, file active (la campagne), angles morts (A320) pour S443–S444 ; `Registres` : dernier lot S445, le prochain au plus tard en S448.
- **P3** — départ à plat, 5 s, `|u′|` max (part de `aω`) : 25 cm — 1,25 cm **57 %**, 2,5 cm **56 %**, 5 cm **54 %** ; 12,5 cm, 5 cm —
  **39 %** (facteur 1,37). **∝ `a`**, peu sensible à la maille : ni une physique du second ordre, ni la seule lecture de la surface (le
  facteur 1,5 manqué) — **une erreur du premier ordre du schéma discret appliqué à B** (la surface que les particules transportent ne
  suit pas exactement la surface analytique), au nombre d'onde et à la fréquence de B, qui force δ en résonance ; le pas couplé l'évite
  parce que sa surface de B est analytique. **c1 non reçu** ; seconde session : **le plafond (ADR-213 D2)**. **La voie suivante** :
  l'eau totale dans la bande — ce qu'`Apic3` fait déjà (le témoin de S444 suit la houle à 1,2 mm) — et B à sa seule frontière, le
  raccord avec la mer relative (`Step3`). Le mode relatif d'`Apic3` reste, éteint par défaut, comme instrument.
- **P4** — APIC-CARTE §23.2, note datée d'ADR-214 ; journal ; jeton libre ; maillons 2 (justifiés : S406) ; suivant : S446, c2 — la conception du raccord.

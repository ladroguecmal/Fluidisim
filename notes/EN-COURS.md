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

Session : S669 — **en cours**. En autonomie vers la v2 ; 2.7 (« manquent… la dissipation au déferlement »). `Cote2D` s'arrête à 2 m de
fond, avant que la mer de S667 ne déferle : sans dissipation, la levée croît sans borne vers le rivage.

**Ce que la session fait.** **Le déferlement d'une mer dans la marche parabolique** : toutes les composantes marchent ensemble, rangée
par rangée (`propager_spectre_periodique`). En chaque nœud, la mer entière donne `Hrms = 2·√(Σ (a_c·|A_c|)²)`, et Battjes et Janssen
(1978) donnent `D/E = 2α·f̄·Q_b·(H_max/Hrms)²`, avec `H_max = 0,88/k̄·tanh(γ·k̄·h/0,88)` et `(1 − Q_b)/ln Q_b = −(Hrms/H_max)²`. Chaque
composante est amortie au même taux (Chawla, Özkan-Haller et Kirby 1998, REF/DIF-S) : `A_x` reçoit `−(w/2)·A`, `w = (D/E)·ω/(p·k_x)`.
`γ` vient de Battjes et Stive (1985) : `0,5 + 0,4·tanh(33·s₀)`. La marche de `marche_grand_angle` devient un état qui avance d'une
rangée, sans changer son arithmétique.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268)

- **témoin** : l'empreinte de trois marches existantes (Berkhoff linéaire et non linéaire, la plage périodique), prise avant le
  remaniement : `4a10864d51e1108b`.
- **instrument** : l'équilibre d'énergie 1D de la même mer (chaque composante réfractée par Snell, `dF_c/ds = −(D/E)·E_c`), intégré à
  part (RK4 au mètre) dans l'essai. Ce qui départagerait : une dissipation juste suit la référence à la précision de la marche sans
  déferlement ; un `Q_b` inversé ou un taux mal rapporté au flux s'en écarte de plusieurs dizaines de % (au calcul, avec la bissection
  inversée : `Hrms` 0,20 m au lieu de 0,89 m à 2 m de fond).
- **calcul** (scratchpad `s669_bj.py`) : `f̄` = 0,106 Hz, `Hrms₀` = 1,55 m, `γ` = 0,640 ; le déferlement commence vers 6 m de fond
  (`Q_b` 0,006) ; à 2 m, `Hrms` 0,89 m, `Hrms/h` 0,45, `Q_b` 0,21. Le plancher de l'instrument : la marche sans déferlement s'écarte de
  la côte 1D de 0,6 % en amplitude par composante (S665), d'où ≈ 0,6 % sur `Hrms`, plus le retard d'une rangée du taux (`w·dx` ≈ 0,1
  au plus fort) — d'où la borne de **3 %**.
- **ADR** : ADR-196, ADR-259 (le déferlement localisé, la dissipation au tableau 2.7), ADR-268.
- **pièges** : la bissection de `Q_b` (son sens, vérifié au calcul) ; `E` en `a²/2` et `Hrms² = 8E` (une seule onde : `Hrms = 2a`) ; la
  dissipation rapportée au flux normal (`c_g·cos θ`, ici `p·k_x/ω`) ; la profondeur sous 1 m, refusée par la marche.

**Critères, écrits avant.** (1) Le remaniement au bit : l'empreinte inchangée ; la marche spectrale sans déferlement, au bit des marches
séparées. (2) La mer de S667 sur une plage de 80 m à 1 m de fond : `Hrms(s)` de la marche à moins de **3 %** de la référence 1D, à
chaque rangée. (3) Rapportés : l'écart sans déferlement à 1 m (la levée sans borne), et `Hrms/h` au rivage — entre 0,35 et 0,55 (le
champ de Thornton et Guza (1982), ≈ 0,42, cité de mémoire : un repère de vraisemblance, pas une mesure).

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — la marche par rangée ; `propager_spectre_periodique` ; l'essai ; (1)–(3).
- [ ] **P3** — preuve ; liste 2.7 ; rituel.

### Notes de reprise

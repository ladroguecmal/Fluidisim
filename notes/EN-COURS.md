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

Session : S322 — **terminée** (2026-09-22 21:16, P6 reportée). **Lot 2, A289** : la croissance de δ sous une houle B, contre le pas
de temps à maille fixe — l'essai qu'il faut faire **avant** l'arbitrage de l'utilisateur.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée : *« Continue »*, après S321 ; suite proposée par S321 ([bilan](../docs/registres/BILAN-GLOBAL-S321.md)
§7) et par la lecture de `simufluid` (S320, [§1](../docs/registres/LECTURE-SIMUFLUID-S320.md)).

**Ce que la session doit rendre possible.** Savoir si la croissance de S319 est, au moins en partie,
un **défaut d'intégration du pas couplé** — elle dépend alors du pas de temps à maille fixe, comme
chez `simufluid` — ou une **propriété du modèle** — elle n'en dépend pas. Consommateur : le choix de
la voie d'A289 par l'utilisateur ; si la cause est numérique, ajouter la dispersion d'amplitude à B
ne la soignerait pas. Puis l'ordre E.

**Le banc.** E1 de S319 ([preuve](../docs/validation/MER-S319.md)), `transfert_oriente mer` : houle
d'une composante, 5 cm, λ = 4 m, `T` = 1,6 s ; δ nul au départ ; **maille fixe, 25 cm** ; pas de
20, 10, 5 et 2,5 ms, soit 80 à 640 pas par période. Option `MER_DT_US`, défaut 10 000 : **sans
elle, le banc de S319 est inchangé au bit**. Mesure : `δ_max` chaque seconde ; **taux** = pente de
`ln δ_max` entre 10 et 30 s, dans la phase exponentielle de S319.

**Prédiction, écrite avant la mesure.** Si A289 domine — une onde totale que B linéaire ne suit
pas —, le taux ne dépend pas du pas : écart **< 10 %** entre 20 et 2,5 ms. Si le pas couplé y
contribue, l'écart dépasse **30 %**, et le taux peut changer de signe. Entre les deux : indéterminé,
un point de plus.

Critères :
1. À 10 ms, la trace reproduit S319 **au bit** : 2,83 cm à 20 s.
2. Taux publiés aux quatre pas ; la règle ci-dessus appliquée telle qu'écrite.
3. Rien dans le cœur ; aucune réception touchée.

### Plan

- [x] **P1** — jeton, plan seul. *(Committé avec `[>]` — même oubli qu'en S321 ; coché en P2. Écrire P1 déjà coché dans le plan.)*
- [x] **P2** — `MER_DT_US` et trace à la seconde ; non-régression à 10 ms sur 20 s.
- [x] **P3** — les quatre pas, 40 s, en parallèle ; taux et verdict selon la règle.
- [x] **P4** — *si la règle dit « indéterminé »* : le point manquant, déclaré avant. **Sans objet** : la règle tranche.
- [x] **P5** — preuve : section datée de [MER-S319](../docs/validation/MER-S319.md) — un fil, une
  preuve (ADR-187 D7) —, avec « Reproduire » ; file, A289.
- [ ] **P6** — S320 P5b : §5 bis de B10 au retour du calcul lancé à 20:11 — asynchrone. **Reportée** :
  encore en calcul à 21:16 ; le point daté de la file (S321) la porte.
- [x] **P7** — rituel.

### Notes de reprise

**P2 (21:07).** Non-régression à 10 ms, 20 s, 25 cm, houle 5 cm — sortie brute gardée ici, faute
de l'avoir été en S319 : `perturbation_max_m=2.828526496887207e-2 recu_droite_m3=3.4107479323174636e-2
bilan_final_m3=-6.788354459604101e-4 recu_droite_sur_paquet=292.1912` ; publiés en S319 : 2,83 cm,
3,4·10⁻² m³, 6,8·10⁻⁴ m³, 292 — **identiques aux chiffres publiés**, seuls comparables. 56 s de calcul.

**P3 à P5 (21:14), fusion déclarée au commit.** Taux 0,1015 / 0,1012 / 0,1009 / 0,1007 s⁻¹ à 20 / 10 / 5 / 2,5 ms, R² 0,995 ; écart 0,8 %.
Calcul 59 / 112 / 205 / 378 s, quatre processus en parallèle avec P5b.

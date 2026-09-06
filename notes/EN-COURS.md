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

```
Session          : S22
État             : en cours
Battement        : 2026-09-06
Objectif         : C01 — le repos hydrostatique sur pente, et le premier δ
```

### Plan

C01 est décrit dans `CAS-CANONIQUES` comme « le test le moins spectaculaire, le plus rapide, et
celui qui élimine le plus de candidats ». Il n'a besoin d'aucune houle, d'aucun corps, d'aucun
réseau : de l'eau au repos sur un fond incliné, et la question de savoir si elle y reste.

**Ce que cette session ne fait pas.** Elle ne choisit pas le solveur δ du projet — ce choix est le
banc **B3**, et ADR-007 §5 liste cinq candidats sans en privilégier aucun. Ce qui est écrit ici est
un **véhicule d'essai**, étiqueté comme tel, exactement comme `background.rs` l'est pour `B` : il
donne à C01 quelque chose à faire tomber. Le livrable durable est le **cas**, pas le solveur.

*Thèse déclarée avant l'exécution : le schéma évident échoue C01.* Le gradient de pression et le
terme de fond sont deux discrétisations différentes de la même quantité ; sur un fond incliné elles
ne s'annulent pas, et l'eau au repos se met à couler. Si la thèse est fausse, c'est mon montage qui
est trop facile, pas le schéma qui est bon — et il faudra le dire.

- [x] **P1** — plan, jeton.
- [x] **P2** — `delta.rs` : grille 1D, état conservatif `(h, hu)`, flux de Rusanov, pas de temps
      CFL. Fond plat d'abord, où le repos est trivialement exact. Test de repos sur fond plat.
- [x] **P3** — le terme de fond au premier jet, la pente 1:20, et C01 branché dans le mode
      `physics` : `max|u|` et `max|η − η₀|` mesurés sur le champ après 60 s.
- [ ] **P4** — exécuter, constater, **mesurer** l'amplitude du courant parasite. Un chiffre, pas
      une impression.
- [ ] **P5** — reconstruction hydrostatique (Audusse) : le schéma équilibré. Réexécuter, comparer
      les deux chiffres dans le même rapport.
- [ ] **P6** — **ADR-030** : ce que C01 a appris, et pourquoi « équilibré sur fond variable » est un
      critère d'**élimination** pour B3, connu avant le banc et non découvert pendant.
- [ ] **P7** — répercussions : `CAS-CANONIQUES`, `cas_en_attente()`, index, angles morts, notes
      correctives, décomptes.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

#### Le dépôt a forké une seconde fois — constaté à l'ouverture de S22

`git worktree list` et `git branch -a`, les deux commandes qu'`CLAUDE.md` impose, ont montré ceci :

| Ligne | Sessions | Contenu propre |
|---|---|---|
| `master` (et `claude/reprise-projet-s22-715339`) | S08 → **S17** | la fusion S16, qui a importé l'autre ligne jusqu'à S15 |
| `claude/reprise-projet-5134cd` | S08 → **S21** | ADR-027 à ADR-029, `code/`, H1 et H3 |

Point de divergence commun : `8fe1503` (S07) — **le même fork que `FORK-S08-S15.md` décrit**, jamais
refermé du côté git. La fusion de S16 a été faite **par import de contenu**, pas par un merge : la
ligne source ne l'a donc jamais reçue et a continué seule pendant quatre sessions.

**Ce que S22 a fait :** repartir de `a6cfe6f` (S21, la ligne la plus avancée et la seule dont le
`REPRISE.md` annonce S22) sur une branche `claude/s22-suite`, **sans rien réécrire**. `master` est
intact.

**Ce qui reste à trancher, et qui n'est pas à moi :** que faire du travail propre à `master`,
S16-S17 — la carte de renumérotation, la revue de cadence sur les documents importés. Il n'est pas
perdu ; il n'est pas non plus dans la ligne vivante. Angle mort à enregistrer en P7.

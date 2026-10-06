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

Session : S573 — **terminée**. En autonomie, **7.7** : deux manques de S572.

**Ce que la session fait.** (a) **L'invalidation** (SPEC-006 §5.4) : `invalider(tuile, cellules)` — une commande de V à l'amont fait passer
la tuile en cadence `Immediat` et toutes ses prévisions à `CrossCause::Aucune` (« je ne sais plus » : un résultat, pas un échec) jusqu'à
la publication suivante qui reçoit une prévision établie. (b) **La praticabilité par agent** (ADR-018 §2) : `Agent::Humanoide` (moins de
1,30 m et `HR` sous 1,25), `Agent::Vehicule { gue }` (la profondeur sous le gué de son châssis), `Agent::Bateau { tirant, marge }`
(`profondeur − tirant − marge > 0`) ; `prochain_changement(agent, profondeur(t), …)` — le prochain franchissement du seuil de cet agent,
par le balayage et la bissection de S570, généralisés à une liste de seuils.

**Références, calculées avant** (ce script les écrit). La marée de S570 (`0,8 + 0,4·sin`, 12,42 h). Un véhicule au gué de 0,6 m, depuis
la mi-marée descendante : praticable dans **3726.000 s** (`T/12`), en descendant. Un bateau de 0,9 m de tirant avec 0,2 m de marge
de houle, depuis la mi-marée montante : navigable dans **6034.925 s** (`asin(0,75)·T/2π`), en montant. Un humanoïde : la marée
culmine à 1.2 m, sous la nage — aucun changement.

**Quantum** (ADR-236 D1) : la bissection à 1 ms. **Critères, écrits avant.** (1) les deux délais à 10 ms, le sens ; l'humanoïde `None` ;
(2) `praticable` aux bornes : véhicule à 0,6 m praticable (borne incluse), à 0,601 m non ; bateau à 1,1 m non, à 1,101 m oui ; humanoïde à
0,5 m et 2 m/s (`HR` = 1,25) non ; (3) l'invalidation : après elle, cadence `Immediat`, 256 causes `Aucune` ; republiée sans prévision,
toujours `Aucune` ; avec prévision, `Maree` ; (4) S570 et S572 inchangés.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — l'invalidation, les agents ; (1)–(4).
- [x] **P3** — preuve ; liste 7.7 ; rituel.

### Notes de reprise
- **P2 fini** — véhicule 3 725,9999 s, bateau 6 034,9260 s, humanoïde `None` ; bornes ; invalidation 128 → 256 `Aucune` → 128 rétablies.
  En route : la praticabilité du bateau et sa prévision ne lisaient pas le même seuil en f32 — corrigé. Suite 747.
- **P3** — preuve TRAVERSABILITE-AGENTS-S573 ; liste 7.7 ; index ; journal.


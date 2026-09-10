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

Session : S156 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S155-1. La première branche que S154 proposait, que le refus du noyau rendait
inaccessible et que ADR-106 vient d'ouvrir : **bilan énergétique d'un sillage prolongé et domaine
de collecte requis**, comme S153 et S154 l'ont fait pour les impacts.

### Plan

- [x] **P1** — état réel, jeton, plan déclaré et committé seul.
- [ ] **P2** — énergie d'un sillage prolongé jusqu'à 60 s : puissance pendant le forçage,
      conservation après extinction. C'est exactement ce que S155 vient de rendre mesurable, et
      personne ne l'a encore regardé au-delà de 8 s.
- [ ] **P3** — **où** est cette énergie dans l'espace à 60 s : balayage radial du champ contre
      l'oracle f64 à quadrature doublée, et fraction contenue dans un rayon donné.
- [ ] **P4** — décider d'après les chiffres : la borne utile est-elle en temps, en domaine, ou
      en **résolution spectrale** ? Les trois ne se corrigent pas au même endroit.
- [ ] **P5** — recevoir ce qui doit l'être, avec un test témoin ; ne rien construire dont le
      prix dépasse le bénéfice.
- [ ] **P6** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ d7db1d2 = master, trois copies coïncidentes, rien en attente.
297 tests/cinq ignorés, 106 ADR, 213 angles, 232 leçons, 18 invariants.

Outillage déjà en place, à ne pas réécrire :
- `examples/wake_motion.rs` (S150) monte la chaîne complète — `Wake::build`, WPRS, journal,
  `Prepared::from_journal`, oracle f64 `GaussianPressure` à quadrature doublée ;
- `Prepared::energy_j()` et `power_w()` donnent le bilan spectral en joules ; l'énergie est une
  somme de Kahan sur les nœuds, `rho/2 * poids * (g|eta|^2 + |v|^2/k)` ;
- `wake_emitter` découpe le mouvement en tronçons, chacun source distincte (ADR-104).

**Prédiction écrite avant la mesure**, pour qu'elle puisse être démentie — c'est ce qui a le
mieux rapporté en S155. Le paquet s'étale à la vitesse de groupe `c_g = ½ sqrt(g/k)` : pour
sigma 1 m et cutoff 6, les modes rapides sont les **petits** k, et à 60 s ils sont déjà à des
centaines de mètres. Mais la quadrature est **discrète** : un pas radial `dk ≈ cutoff/radial`
rend le champ périodique de période `2*pi/dk`, soit ~134 m pour radial 128. Si le paquet dépasse
cette période, il **revient** par l'autre bord au lieu de partir. La borne utile serait alors ni
le temps ni le domaine, mais la **résolution spectrale**, et le rayon honnête décroîtrait
relativement à l'étalement au lieu de croître.

Si c'est vrai, ADR-106 est nécessaire et **non suffisant** : porter l'horizon à 64 s sans porter
la résolution ne donne qu'un champ replié. Si c'est faux, il faut le dire aussi.

Piège identifié d'avance : l'oracle `GaussianPressure` porte **la même** discrétisation, en plus
fin. Un accord candidat/oracle ne prouverait donc rien sur le repliement — les deux replient. Le
seul juge est la comparaison entre **deux résolutions** et, si possible, une quantité physique
indépendante de la quadrature : l'énergie totale, qui doit être conservée après extinction.

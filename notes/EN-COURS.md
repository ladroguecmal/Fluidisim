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

Session : S121 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : A197 — un même refus confond « point hors domaine », imputable à l'appelant, et
« champ dégénéré », défaut de la couche. Le second se reproduit partout ; le premier se
corrige en changeant de point. Un appelant qui filtre — ce qu'ADR-080 rend naturel —
masquerait le second en croyant écarter le premier.

### Plan

- [x] **P1** — amorce, jeton, plan.
- [ ] **P2** — inventaire, avant toute décision (L209) : recenser dans le crate les refus qui
      confondent une faute d'appelant avec un défaut de couche, **et vérifier si le cas
      dégénéré est seulement atteignable** — S120 a annoncé ce montage sans le construire.
- [ ] **P3** — ADR-081, sur ce que l'inventaire aura montré.
- [ ] **P4** — construire la séparation et mettre à jour les appelants.
- [ ] **P5** — le test qui compte : un champ réellement dégénéré, et la démonstration qu'un
      filtrage par `admits` ne l'écarte plus en silence.
- [ ] **P6** — campagne, hachages inchangés, mise en régime (A195).
- [ ] **P7** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ c0b491a = master ; trois copies coïncidentes, 5134cd archivée, c107bf sur la ligne S44.

Point d'entrée exact : `RadialImpact::sample` (`code/water-core/src/radial_impact.rs`) rend
`Error::Domain` à la fois pour une position hors domaine et pour une sortie non finie — ce
dernier cas est le bloc de test de finitude en fin de fonction. `mixed::sample_world_batch`
mappe ensuite **toute** erreur de champ vers `composition::Error::Domain` ; ce mappage masque
aussi `Error::Time` du champ, aujourd'hui neutralisé en amont par `state`.

Question à trancher en P2 avant toute construction : **le bloc de finitude est-il atteignable ?**
Si `WaveEvent::impact` et `RadialImpact::new` bornent assez fort, il ne l'est pas, et la réponse
juste n'est plus « séparer deux causes » mais autre chose. Ne pas présumer — S120 a précisément
annoncé un montage dégénéré qu'elle n'a pas construit, et l'a corrigé par note datée.

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

Session : S136 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : A200, sévérité 1 — un objet qui tombe dans l'eau doit produire les deux nombres que
`WaveEvent::impact` exige, `wavelength_m` et `energy_j`. Aucun document ne dit comment.
**Session de conception.**

### Plan

- [x] **P1** — état réel, jeton, plan seul.
- [x] **P2** — séparer ce qui est dérivable de ce qui doit être calibré. Deux nombres, deux
      statuts différents, et c'est le cœur de la session :
      - **λ** : ADR-083 pose `λ = α·b` avec α « à calibrer ». Mais α pourrait n'être pas un
        paramètre libre : le modèle d'ADR-060 fixe déjà la forme spatiale initiale
        `η(r) = ∫A(k)J0(kr)k dk` pour une bande donnée. Mesurer le **rayon caractéristique**
        de cette forme en fonction de λ donnerait α par dérivation interne, pas par arbitrage.
      - **E** : la fraction de l'énergie d'entrée qui part en ondes de gravité est une
        propriété physique externe, que rien dans le modèle ne peut produire. Elle restera
        « à calibrer » — mais S123 en a mesuré une **borne supérieure**.
- [x] **P3** — ADR-092 : α dérivé à 3,35 (borné [3,35 ; 6,11]), η calibrable mais borné.
- [x] **P4** — module `impact_generator` : η en paramètre de l'appelant, borne du modèle appliquée.
- [x] **P5** — borne annoncée vérifiée contre le candidat (97 % / 105 %) sur douze
      combinaisons ; lois d'échelle vérifiées ; onze cas de S123 rejoués.
- [x] **P6** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ 34d1be8 = master, trois copies coïncidentes.

Ce dont on dispose, avec provenance :
- **SPEC-001 §5 bis** (Wagner) : demi-largeur mouillée `c(t) = (π/2)·v·t / tan β`, durée
  d'impact `t_impact = 2·b·tan β / (π·v)`, masse ajoutée `m_a = ½πρc²` par mètre, impulsion
  `J = Δ(m_a)·v_rel`. C'est ce qui relie un objet à l'eau qu'il déplace.
- **ADR-060** : `A(k) = C x²(1-x)²` sur `[k0/2, 2k0]`, `k0 = 2π/λ`, et `C` déduit de l'énergie
  par Parseval radial. La forme spatiale initiale est donc entièrement déterminée par λ et E.
- **ENVELOPPE-IMPACTS-S123 §4.3** : le candidat ne porte que 10⁻⁶ à 10⁻² de l'énergie de
  référence avant que la pente dépasse la limite du milieu. C'est une borne sur ce que le
  générateur peut demander.
- **SPEC-001 §5** : `λ = 2πv²/g` — sillage d'un mouvement établi, écartée pour une entrée
  (ADR-083), et il faut continuer de l'écarter.

Piège à éviter : produire une fonction qui rend des nombres d'apparence physique alors qu'un
facteur reste arbitraire. Si η n'est pas calibré, il doit être **un paramètre de l'appelant**,
nommé et borné, pas une constante enfouie.

P2-P6 : ADR-092, GENERATEUR-S136, suivi daté sur ENVELOPPE-IMPACTS-S123, suivi A200, L214,
journal, index, README, REPRISE, jeton rendu, ff-only. 269 tests/cinq ignorés, hachages inchangés.

Pour S137 sans relire : S136-1 est la spécification du banc B2, cité par ADR-060, ADR-083 et
ADR-092 sans avoir jamais été décrit. Ce qu'il doit fixer est désormais précis — `α` dans
[3,35 ; 6,11] et `η` sous `2Kgα⁴bs²/v²`. Deux questions à trancher : quelle observable mesure
α (le rayon de la perturbation initiale n'est pas directement observable sur une vidéo ; le
spectre des ondes à distance, peut-être), et quelle observable mesure η (l'énergie des ondes
loin de l'impact, contre l'énergie cinétique de l'objet). SPEC-003 et PLAN-BENCHMARK décrivent
la forme des autres bancs — s'y conformer plutôt qu'inventer un format.

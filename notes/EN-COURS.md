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

Session : S129 — en cours
Agent : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Objectif : S128-1 — recevoir énergie cinétique et bilan total du candidat N256/R80/48,
depuis ses nœuds effectivement construits, contre les résultats indépendants S127.

### Plan

- [x] **P1** — état réel, jeton et plan seul ; continuité de S128 sur master propre.
- [ ] **P2** — instrument dans les tests privés, sans exposer les nœuds en API : densité
      positive profonde, termes croisés conservés, potentielle/cinétique séparées ;
      intégration Simpson320/640, disque80 et anneau32–80. Références chiffrées S127 figées.
- [ ] **P3** — réception à0/24/48 s, témoins initiaux et contre-épreuves (cinétique omise,
      termes croisés omis), contrôles de surface contre sample, rapport fidèle aux résultats.
      Corriger la production seulement si un défaut est établi.
- [ ] **P4** — tests adaptés et suite complète (nouveau test), journal/registres/actions,
      index/README/passation, décomptes/invariants, jeton libre et commit final.

### Notes de reprise

Départ7701759 master propre ; branches/copies anciennes sans avance ni modification, aucune
copie créée. Instructions et corpus déjà lus dans cette conversation, reprise immédiate.
S78 physical_disk est privé dans les tests, assemble en f64 les nœuds réels/PhaseQ32/Bessel.
La même méthode sera employée en S129 ; pas de fonction d'énergie de production ajoutée.
Référence indépendante S127 publiée à10 décimales pour E/E0, cinétique=total−potentielle ;
son incertitude de copie <1e-10 E0, largement sous le critère annoncé. Pas d'oracle du candidat.
Critères AVANT mesure : total/potentielle/cinétique et anneau contre S127 <=1e-4 E0 ;
raffinement radial <=0,002 E0, énergie totale fine proche deE0 <=0,003 (S78/S127).
Densité >=−1e-12 J/m², cinétique initiale nulle, énergie hors32 à48 >0,5 E0 et initiale<0,003.
Contrôle de surface assemblée depuis les mêmes nœuds <=1e-6 normalisé aux poids positifs,
plus strict que S126 (1e-4) car seules les opérations d'assemblage diffèrent.
Références : TRANSPORT-ETENDU-S127 tableaux ; conserver les termes croisés et la profondeur
infinie du modèle. Refuser toute lecture de ce bilan comme une réception en profondeur finie.

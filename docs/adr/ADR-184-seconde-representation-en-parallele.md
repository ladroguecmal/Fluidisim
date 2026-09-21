# ADR-184 — La seconde représentation avance en parallèle du lot 2, par sessions alternées

- **Statut : actée**, S316, 2026-09-21, **décision de l'utilisateur** (« je suis ta
  recommandation »), en cours de session.
- **Complète** [ADR-178](ADR-178-strategie-en-trois-systemes-physiques.md) D7, dont elle modifie
  **l'ordre** et rien d'autre : les sept lots, leurs critères d'arrêt et le périmètre restent.
- **Ne tranche pas** la représentation elle-même : particules sur grille, particules pures ou
  surface implicite. Ce choix reste celui de l'utilisateur, sur une comparaison chiffrée.

## 1. Ce que l'utilisateur a écrit

> « J'ai peur que l'on ne se soit pas compris : dans la zone δ, il s'agit comme d'une simulation de
> billes ou d'autres types qui permettent d'avoir plusieurs particules d'eau sur la même
> ordonnée. »

> « Est-ce que c'était indiqué dans le projet ? » — puis : « on laisse le plan initial en route ».

> « Peut-être que le développement de ce système peut se faire en parallèle. Réfléchis à savoir
> si tu as besoin d'éléments qui ne peuvent pas être décidés maintenant. » — puis, après la
> réponse : « Je suis ta recommandation. »

## 2. Ce qui était déjà écrit, et ce qui ne l'était pas

**L'exigence était là dès l'origine.** Les sources demandent qu'une région chaotique — impact,
splash, rouleau — soit portée par des éléments de fluide qui se subdivisent
([architecture](../sources/systeme_eau_architecture_globale.md) §12), et constatent qu'*« un
heightfield ne peut pas avoir deux hauteurs pour une même paire (x,y) »*
([topologie](../sources/guide_topologie_ocean_haute_mer_plage.md)). ADR-001 a rejeté le « tout en
champ de hauteur » parce qu'il *« interdit rouleaux, cavités, poches d'air et immersion,
explicitement exigés »*.

**Ce qui a été choisi ensuite est un ordre, pas un renoncement.** ADR-175 D5 (S294) a démarré la
3D par une surface à **une hauteur par colonne**, prolongement du δ 2D reçu, et a renvoyé cavité,
jet et déferlement à une **seconde représentation**. ADR-178 l'a placée au **lot 5**, après les
corps flottants, donc après la v1. Depuis S294, tout le travail sur δ a porté sur la première
représentation — d'où la question de l'utilisateur.

## 3. Décisions

**D1 — Le lot 5 est préparé en parallèle du lot 2, par sessions alternées.** Après S316, une
session sur le lot 2 (ordre D, sur décision de l'utilisateur à la sortie de l'ordre C), une session
sur le lot 5, et ainsi de suite. **Jamais deux sessions simultanées** : le jeton de `REPRISE.md`
n'est pas un verrou entre copies, et le dépôt a forké trois fois ainsi (L137, AGENTS.md).

**D2 — Le lot 5 commence par une comparaison chiffrée**, pas par une construction : particules sur
grille (FLIP/APIC), particules pures (SPH), surface implicite (ensemble de niveaux). Deux cas :
l'entrée d'un objet dans l'eau (B10 : couronne, cavité, pincement, jet de Worthington) et une
rupture de barrage contre un obstacle. Critères : conservation du volume sous le compteur du
lot 1, fidélité aux références de B10, coût, raccord possible à B/W et au δ en colonnes. **Le choix
revient à l'utilisateur**, sur cette comparaison.

**D3 — Le lot 5 ne dépend ni du lot 3 ni du lot 4.** L'objet qui entre dans l'eau est
**cinématique** — trajectoire imposée — tant que les corps rigides n'existent pas. La grille MAC,
la multigrille et le compteur de volume du lot 1 se réutilisent.

**D4 — Le chantier interne est nommé** : la condition de surface libre du solveur est construite
colonne par colonne (fantômes « au-dessus » et latéraux, `delta3d_mobile.rs`). Une eau à plusieurs
couches demande une frontière liquide/air **dans toutes les directions**. C'est une décision
technique de la session, pas de l'utilisateur.

**D5 — Ce qui ne se décide pas maintenant, et qui le décidera :**

| élément | décidé par | quand |
|---|---|---|
| la représentation | l'utilisateur | sur la comparaison de D2 |
| les impacts qui comptent pour le jeu (objets, tailles, vitesses) | l'utilisateur s'il le sait ; sinon **hypothèses déclarées** | avant de dimensionner |
| la frontière entre eau physique et effet visuel (gouttes, embruns) | proposée par la session, validée par l'utilisateur | avec la comparaison |
| le critère de bascule colonnes → seconde représentation | une **mesure** (ADR-112 : aucun seuil valable aujourd'hui) | après la comparaison ; d'ici là, domaines à plusieurs couches par nature |
| les références visuelles | l'utilisateur (REVUE-VISUELLE) | quand un rendu existera |

## 4. Ce qui devient faux si cette décision est mal lue

**« En parallèle » ne veut pas dire « avant la v1 ».** La v1 reste la porte D franchie (ADR-174
D4), et les lots 3 et 4 gardent leur place ; le lot 5 **avance** à côté au lieu d'attendre.

**« Commencer le lot 5 » ne veut pas dire « choisir les particules ».** D2 commence par comparer.

**Et rien n'est retiré du lot 2** : l'ordre C se termine, l'ordre D reste soumis à la décision de
l'utilisateur, et le volume net reste intégralement dans `pending` (ADR-183 D7).

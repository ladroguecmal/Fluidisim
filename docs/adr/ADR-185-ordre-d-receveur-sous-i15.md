# ADR-185 — L'ordre D : le volume net de δ reçoit un receveur, et ce receveur n'est pas répliqué

- **Statut : actée**, S317, 2026-09-21. L'**ouverture** de l'ordre D est une **décision de
  l'utilisateur** (*« On passe à l'ordre D »*) ; les décisions D2 à D8 appliquent les invariants
  et relèvent de l'autonomie technique déléguée (S71).
- **Applique** [ADR-181](ADR-181-conservation-transfert-oriente-et-ordre-du-lot-2.md) D1 à D4 et
  D10 (ordre D), [ADR-179](ADR-179-tolerances-de-conservation-et-grandeur-restituee.md) D3 et D4,
  [ADR-180](ADR-180-retour-delta-w-et-conservation-du-volume.md) D1 et D2, sans en remplacer
  aucune. **Ne touche pas** au périmètre (ADR-127).
- **Le transfert δ → W reste « partiel »** (ADR-183 D6) : l'utilisateur a ouvert l'ordre D sans se
  prononcer sur ce qualificatif, et la lecture prudente le garde.

## 1. Le problème que l'ordre D rencontre avant tout code

ADR-181 D1 désigne le receveur du volume net : **V** pour un contenant, un **niveau moyen régional
de B** pour l'eau ouverte, **jamais W**. Mais trois invariants disent qui a le droit d'écrire dans
ces couches :

- **I-04** — δ n'a jamais d'autorité ; il est calculé sur le client, sans reproductibilité ;
- **I-03, I-10** — B et V sont déterministes et répliqués ; V est tenu par le serveur ;
- **I-11** — *aucun chemin d'énergie ne va du client vers le monde répliqué* ; et **I-15**, qui se
  teste *« plutôt que de faire l'objet d'un arbitrage »* : une grandeur n'est autoritaire que si
  tous les participants la calculent à l'identique depuis des données répliquées.

Un volume **issu de δ** versé dans l'état **répliqué** de B ou de V ouvrirait exactement le chemin
qu'I-11 interdit. ADR-181 D1 nomme donc la **couche** ; I-15 fixe le **statut** du receveur.

## 2. Décisions

**D1 — L'ordre D est ouvert** — décision de l'utilisateur du 2026-09-21. Son objet est celui
d'ADR-181 D10 : *« développer la comptabilité et la restitution effective du volume net à travers
B ou V, suivant l'environnement »*.

**D2 — Un volume issu de δ n'entre jamais dans l'état répliqué de B ni de V.** Ce n'est pas une
restriction de l'ordre D : c'est I-11, et aucune décision de session ne l'amende.

**D3 — En eau ouverte, le receveur est le niveau de B *tel que ce client le représente*.** Une
correction **locale**, non répliquée, dérivée des seuls pas de δ de ce client — exactement le statut
de `W_local` pour W (I-11). Elle ferme le **bilan de la représentation** : ce que le joueur voit ne
crée ni ne détruit d'eau. Elle ne change pas le monde : le niveau autoritaire de B reste sa donnée
cuite (`HydroSample`, marée comprise, I-02).

**D4 — Ce niveau est porté par une région identifiable, attachée à une surface de contrôle.**
ADR-181 D2 interdit *« une quantité locale d'eau répartie sur un océan infini »*. La région est donc
**déclarée** : un rectangle du repère de la cellule, adossé au segment de la ligne de contrôle dont
il reçoit, de profondeur **déclarée et publiée** ; son niveau est le volume reçu divisé par son aire.
Une région sans aire, non finie, ou qui ne touche pas sa ligne **se refuse**. Un scalaire global
n'est pas un cas limite de cette forme : il en est exclu **par construction**.

**D5 — En contenant, c'est δ qui s'asservit au bilan de V, pas V qui reçoit de δ.** La porte E le
dit déjà — *« la comptabilité de masse est identique avec et sans δ »* (C21) : V est autoritaire,
calculé sur le serveur, et δ n'est qu'une représentation de sa surface. Rien de δ n'entre dans V.
L'articulation V↔δ est la porte E ; cette session en écrit la règle, pas le mécanisme.

**D6 — La grandeur reçue est le flux de perturbation sortant à la surface de contrôle intérieure**
(ADR-179 D3), **signé**, net de ce que W porte effectivement (`transferred`, nul pour le volume).
Ce que la bande B/W pousse dans δ (`band_in`) ne se restitue jamais (ADR-179 D4). Le cas où cette
bande traverse elle-même la ligne de contrôle est **l'ordre E** ; il est nommé, pas traité.

**D7 — Rien ne se restitue sans receveur, par construction.** Le registre ne peut faire baisser le
volume **en attente** que sur présentation d'un **reçu** que seule la région délivre, pour le volume
qu'elle a effectivement pris. Aucune méthode du registre ne s'appelle « restituer » ; aucune
région ne délivre de reçu sans avoir augmenté son propre volume du même montant.

**D8 — La frontière de la région porte un terme, et il vaut zéro par choix déclaré.** Physiquement,
une anomalie de niveau en eau ouverte ne reste pas en place : elle rayonne en onde longue, à
`√(g·h)`, et se dilue. Cette première construction **garde** le volume dans sa région et publie
**zéro** comme flux de frontière, avec la raison. Le rayonnement est un travail distinct, nommé ici
et daté dans la file.

## 3. Ce que cette décision ne tranche pas

- **Le rayonnement de l'anomalie hors de sa région** (D8), ni sa durée de vie.
- **Le mécanisme V↔δ** en contenant (D5) : porte E.
- **Le cas couplé** où la bande B/W traverse la ligne de contrôle (D6) : ordre E.
- **L'affichage** : la correction locale de niveau entre dans la composition B + W + δ de ce
  client ; la façon dont le rendu la consomme n'est pas décidée ici.
- **La déclaration de conservation** : ADR-180 D1 l'interdisait tant qu'aucun receveur n'existait.
  Un receveur local permet de **calculer** la conservation **de la représentation** ; celle **du
  monde** reste celle des couches autoritaires, et aucun banc ne confondra les deux.

## 4. Ce qui devient faux si cette décision est mal lue

**« Le receveur est local » ne veut pas dire « le volume est cosmétique donc négligeable ».** Il est
compté, publié et reçu ; ce qui est local, c'est son **autorité**, pas sa **comptabilité**.

**« B reçoit le volume net » ne veut pas dire « la marée de B change ».** La donnée cuite n'est pas
touchée ; c'est la représentation de ce client qui se ferme.

**Et « en contenant, δ s'asservit à V » ne veut pas dire « V ignore δ ».** V peut **déclencher** un
domaine δ et lui imposer son niveau ; il ne peut pas **recevoir** de lui.

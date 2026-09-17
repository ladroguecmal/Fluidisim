# ADR-162 — Réutiliser le ciel de banc dans les reflets

Actée S266, 2026-09-17, autonomie S71, après R7 accepté (« Très bien continue »).

## Décision

Le ciel procédural de banc est immuable entre deux changements d'habillage. Le recalculer
neuf fois par fragment dans ADR-161 ne produit aucune information nouvelle. Le cuire dans
une texture cubique HDR, puis l'interpoler aux directions de réflexion, conserve le modèle
d'éclairage à une erreur d'échantillonnage près. Ce n'est pas un changement de physique ni
une augmentation du lissage des vagues.

- Texture `RGBA16Float`, six faces, 512² texels par face (12 Mio), interpolation linéaire,
  sans niveaux de détail : même ciel à deux octaves que la branche claire de S265 ;
  habillage brumeux à sa définition existante. Le soleil et les nuages inclus dans `sky_detail`
  sont cuits ; le terme de reflet solaire additionnel reste analytique.
- Cuisson GPU au premier usage puis seulement si l'habillage change ; ni caméra, ni temps,
  ni vent ne changent ce ciel de banc. Nouvelle ressource persistante de l'hôte seulement.
- `--reflets-filtres` utilise le cache si reçu. `--reflets-directs` conserve le témoin S265.
  Le chemin sans reflets filtrés garde son shader historique. La quadrature 3/5 et la
  fermeture statistique d'ADR-161 restent identiques : leur erreur n'est pas résolue ici.
- Un ciel animé doit invalider le cache à partir de ses propres paramètres avant emploi.
  Aucune promesse de coût pour ce cas, absent du banc actuel.

## Réception et arrêt, avant code

[CIEL-CACHE-S266](../validation/CIEL-CACHE-S266.md) : ancien témoin au bit, sondes directionnelles
dont coutures/pôles, erreur des images à poses fixes, coût avec/sans cache dans le même passage,
coût de cuisson initial séparé. Si 512² échoue en précision, éprouver 1024² (48 Mio) avec les
mêmes seuils. Ne pas relever un seuil après mesure pour retenir le candidat.

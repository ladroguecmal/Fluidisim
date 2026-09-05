# Méthode de travail

Protocole que je suis pour concevoir un système à partir de documents d'intention. Il évolue ;
chaque révision est datée dans `JOURNAL.md`.

---

## Phase 0 — Trier les sources

Séparer trois choses que les documents mélangent presque toujours :

- **exigence** — validée par l'équipe, non négociable ;
- **proposition** — suggérée, souvent par un assistant précédent, jamais validée ;
- **question** — reconnue comme ouverte.

Puis ajouter une quatrième catégorie que les documents ne contiennent jamais :

- **question mal posée** — formulée d'une manière qui empêche d'y répondre.

## Phase 1 — Chercher les impasses, en chiffres

Avant toute proposition, tester l'architecture existante par des ordres de grandeur. Une seule
formule suffit souvent à éliminer une direction entière.

> S01 : `λ = 2πv²/g` a supprimé en une ligne l'idée d'un sillage simulé volumétriquement, que les
> documents sources tenaient pour acquise.

Si aucune impasse n'apparaît, c'est le signe que je n'ai pas assez cherché, pas que le document
est bon.

## Phase 2 — Chercher la décomposition qui dissout

Face à une longue liste de questions ouvertes hétérogènes, ne pas répondre question par question.
Chercher **la** décision qui en rend une fraction caduque.

Indice fiable : plusieurs questions qui semblent indépendantes ont la même cause structurelle.

> S01 : §3, §4, §9, §13, §19, §21 étaient six questions distinctes ; elles avaient une seule
> cause — l'absence de séparation entre le champ de fond et sa perturbation.

## Phase 3 — Dériver plutôt que choisir

Quand une question demande « quel seuil ? », chercher d'abord si le seuil est **dérivable** d'une
grandeur physique ou économique. Un seuil dérivé se défend, se recalcule quand le contexte change,
et n'a pas à être re-débattu.

> S01 : les paliers de confiance de la prédiction ne sont pas choisis, ils sortent de
> `Δ(t) = ½·a_max·t² ≤ R_domaine`.

## Phase 4 — Chercher les contraintes qui ne viennent pas du domaine

Les angles morts les plus coûteux ne sont presque jamais dans la physique. Ils sont dans :

- le **déterminisme** et l'arithmétique flottante ;
- la **latence** (lecture GPU, réseau, threads) ;
- la **précision** numérique aux grandes coordonnées ;
- le **référentiel** (rien n'est immobile) ;
- l'**outillage** et le pipeline d'auteur ;
- les **autres équipes** (audio, IA, terrain, anti-triche) ;
- la **persistance** et la propriété des données dans un monde partagé.

Passer explicitement cette liste, à chaque système conçu.

## Phase 5 — Écrire ce qui peut me contredire

Pour chaque thèse structurante, produire trois choses :

1. l'**invariant** qu'elle impose ;
2. son **point de rupture** — le régime où elle cesse d'être vraie ;
3. le **banc d'essai conçu pour l'infirmer**.

Une architecture sans critère de falsification est une croyance. En S01, c'est le rôle de B4.

## Phase 5 bis — Concevoir l'instrument de mesure, et le laisser remonter

*(ajoutée en S03)*

Avant de considérer une conception comme finie, écrire **comment on saura qu'elle est juste**, puis
laisser les contraintes de cette mesure remonter dans la conception elle-même.

Deux effets, tous deux observés en S03 :

- **La formulation d'une assertion vérifie la valeur.** Écrire la référence d'un test oblige à
  refaire le calcul, et la relecture ne l'aurait jamais fait — c'est ainsi qu'une erreur d'un
  facteur deux a été trouvée dans un document relu plusieurs fois.
- **L'outil de mesure impose des contraintes que l'analyse directe ne trouve pas.** L'exigence de
  milliers d'exécutions rapides a imposé que le système soit instanciable sans le moteur, ce
  qu'aucun raisonnement partant de l'architecture n'aurait produit.

Corollaire de méthode : tout protocole de comparaison doit produire **son propre plancher de
bruit** avant d'interpréter quoi que ce soit.

## Phase 6 — Tracer

Chaque section du document source reçoit un statut et un pointeur. Chaque proposition antérieure
reçoit un verdict explicite, y compris quand le verdict est « la question a disparu ». Sans cela,
l'équipe ne peut pas vérifier que rien n'a été perdu, et le travail devient invérifiable.

---

## Règles d'écriture

- **Aucun nombre sans provenance.** Formule citée, ou étiquette « à calibrer » avec le banc qui le
  fixera. (Invariant I-14.)
- **Chaque ADR se termine par « ce qui reste ouvert ».** Un ADR sans questions résiduelles est
  suspect.
- **Distinguer résolu / dissous / partiel / ouvert.** « Ouvert par décision » est un statut
  légitime et doit être dit.
- **Ne jamais présenter une estimation comme une mesure.** Les ordres de grandeur portent leur
  incertitude dans le texte.
- **Concision.** Un paragraphe qui n'apporte ni contrainte, ni chiffre, ni décision est supprimé.

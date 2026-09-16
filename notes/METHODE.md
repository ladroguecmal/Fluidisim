# Méthode de travail

Révision S227, 2026-09-13, demandée par l'utilisateur lors de l'audit global.
L'ancienne méthode de conception est conservée dans Git à `dfd1507`. Les précautions qui ont
trouvé des erreurs restent ; l'obligation implicite d'en trouver ou d'ouvrir une suite disparaît.

## Choisir un lot utile

1. Partir de la demande actuelle, des intentions et de la file active, puis vérifier le blocage
   dans le **code présent** et son contrat. Ne pas hériter d'une absence sans la constater.
2. Nommer la capacité visée et le consommateur : joueur, hôte, serveur V ou prochaine brique.
3. Écrire le critère de réception et d'arrêt avant le code ou la campagne. Si la réponse ne
   change aucune décision ou intégration, différer cette recherche avec son déclencheur.
4. Déclarer le plan dans EN-COURS selon REPRISE. Le plan doit être assez court pour montrer ce
   qui sera fini ; pas un traité préalable à une correction.

Avant de poursuivre un même sujet une troisième session, comparer sa suite à au moins une
capacité de la file encore absente. Le compteur de REPRISE n'est pas un concours de lignes.
Une grande amélioration locale peut ne plus être prioritaire une fois son usage débloqué.

## Construire et éprouver

| nature du travail | vérification proportionnée |
|---|---|
| Correction de code | cas reproduisant le défaut, témoin nominal, assertions du contrat affecté |
| Propriété numérique ou modèle nouveau | référence indépendante ou identité justifiée ; raffinement des paramètres qui peuvent biaiser le verdict |
| Optimisation | même charge utile, même qualité, coût du chemin consommé ; techniques présentes/absentes et domaine (ADR-131) |
| Intégration | scénario traversant les composants réels, avec refus et reprise si le contrat l'exige |
| Rendu jugé à l'œil | revue de l'utilisateur contre références réelles ([REVUE-VISUELLE](../docs/validation/REVUE-VISUELLE.md)) ; un verdict déclenche une mesure, il n'en tient pas lieu |
| Documentation/procédure | liens, cohérence des états actifs, possibilité réelle d'exécuter la consigne |

Chercher d'abord si la réponse se **calcule** avant de lancer une campagne. Distinguer le modèle,
son implémentation, l'instrument et la métrique. Une conservation ou un hash stable ne démontre
pas une précision physique. Un échec utile reste visible ; ne pas changer le seuil pour le vert.
Le seuil physique vient d'une formule ou d'un banc identifié, jamais d'un chiffre choisi après coup.

Une contre-épreuve se justifie par le défaut qu'elle distingue ; pas de variante automatique à
chaque étape. Rejouer les tests affectés, puis la suite requise avant livraison du code. Éviter de
répéter des campagnes inchangées ou toute la suite après une retouche documentaire ; citer le reçu
antérieur avec sa révision quand il reste applicable. Les exemples ne sont pas tous exécutés par
`cargo test --workspace` : cibler explicitement ceux qui portent la propriété étudiée.

Un changement de hash n'interdit pas un correctif : conserver l'ancien reçu, expliquer les champs
qui bougent et recevoir la nouvelle référence. Un changement de protocole réseau ou de décision
architecturale reste une migration explicitement documentée, pas une mise à jour silencieuse.

## Ce qu'il faut lire

Le socle de reprise est dans REPRISE §3. Pour le lot : contrats, ADR cités et corrections datées,
consommateurs et preuves existantes. Chercher les leçons pertinentes plutôt que lire le registre
entier. Repères utiles : L137 (source unique), L176/L243 (blocage à vérifier), L195 (coût complet),
L244 (couplage), L258 (consommateur), L278 (exposition au biais), L280 (intégration), L310 (couverture).

Distinguer exigence, proposition et question. Les sources sont des intentions et des propositions,
pas un ensemble de contraintes déjà validées. Les décisions ultérieures de l'utilisateur et les
ADR les arbitrent. Les rôles des couches et les invariants ne se réduisent pas en cours de lot.

## Ce qui s'écrit, et où

| information | porteur unique |
|---|---|
| règle opérationnelle de reprise/clôture | REPRISE ; amorce/fermeture des copies dans AGENTS |
| objectif et étapes de la session | EN-COURS |
| décision d'architecture | nouvel ADR ; note datée pour correction factuelle |
| preuve détaillée utile à une décision | document de validation, réutilisable |
| capacité présente et limite | FEUILLE-DE-ROUTE, état remplacé et daté |
| travail à faire, motif et déclencheur | file active de QUESTIONS-OUVERTES |
| histoire et résultat de session | JOURNAL, une entrée concise |
| navigation | index, liens plutôt que récits copiés |

**Aucune obligation de nouvelle anomalie, leçon, ADR ou document de mesure.** Une vérification
sans anomalie est un résultat recevable. Un document de validation n'est nécessaire que si le
journal et les tests ne suffisent pas à transmettre le domaine, la preuve ou la décision.
Clore un lot à son critère d'arrêt ; les questions supplémentaires passent par la file.

Les inconnues externes restent inconnues. Les « autres équipes » ne sont pas des interlocuteurs
présents (ADR-028). Les règles d'autorisation acquises sont dans REPRISE §5 ; ne pas les recréer
à partir d'une réserve historique.

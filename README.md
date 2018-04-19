# HumorLampAPI

Le but de ce projet est de pouvoir déterminer l'humeur du monde, où plus précisement celle
de la franchophonie ainsi que celle de l'anglophonie

Pour ce faire, nous mettons en place deux streams sur l'API twitter, un avec des mots en anglais,
et l'autre avec les mots en français.
Nous comptons le nombre de fois que des mots associés à la peur / tristesse ou la joie sont
mentionnés puis nous prenons le sentiment le plus prévalent pour chaque minute avec un historique de
8 minutes.
Nous nous servons également de l'API twitter pour afficher le Trend le plus utilisé du moment.

Du côté du micro-controlleur, nous avons une LED avec 8 voyants, un bouton, et un display.
La LED affiche l'humeur de chacune des 8 dernières minutes et met à jour la minute en cours toutes
les 5 secondes.

Le bouton sert à basculer entre l'affichage de l'humeur francophone à celle anglophone.

Le display affiche si on est actuellement en train de requêtes les informations françaises ou anglaises
ainsi que le Trend le plus répandu.

Le code couleur est le suivant:

Bleu -> Heureux

Rouge -> Peur

Vert -> Triste

Rose / Violet -> Peur + Heureux

Blanc -> Aucune humeur sort du lot

//! Set of systems that manage graphics part.

use bevy::prelude::Query;

/// Affiche l'ensemble des plateaux et des éléments commun au centre.
pub fn draw_board_system(_query: Query<()>) {}

/// Met en évidence le joueur qui doit jouer.
pub fn draw_current_player_system(_query: Query<()>) {}

/// Affiche l'ensemble du plateau d'un joueur.
pub fn draw_board_player_system(_query: Query<()>) {}
/// Affiche le plateau de construction de tuile (sur le côté) du joueur.
pub fn draw_building_board_system(_query: Query<()>) {}
/// Affiche le plateau (carré) du joueur.
pub fn draw_final_board_system(_query: Query<()>) {}
/// Affiche le cadre de malus du joueur.
pub fn draw_malus_system(_query: Query<()>) {}
/// Affiche toutes les tuiles sur la plateau du joueur.
pub fn draw_player_tiles_system(_query: Query<()>) {}
/// Affiche le score du joueur.
pub fn draw_player_score_system(_query: Query<()>) {}

/// Affiche l'ensemble des éléments commun (au centre du jeu).
pub fn draw_middle_system(_query: Query<()>) {}
/// Affiche les groupes de tuiles au centre du jeu.
pub fn draw_middle_group_system(_query: Query<()>) {}
/// Affiche les tuiles des groupes de tuiles au centre du jeu.
pub fn draw_tile_middle_group_system(_query: Query<()>) {}
/// Affiche les tuiles non groupé au centre du jeu.
pub fn draw_middle_tile_system(_query: Query<()>) {}

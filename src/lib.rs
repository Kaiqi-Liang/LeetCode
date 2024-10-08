#[macro_export]
macro_rules! counter {
    ($iter:expr) => {
        $iter.fold(HashMap::new(), |mut a, c| {
            *a.entry(c).or_insert(0) += 1;
            a
        })
    };
}

mod append_characters_to_string_to_make_subsequence;
mod average_waiting_time;
mod binary_tree_postorder_traversal;
mod combination_sum;
mod combination_sum_ii;
mod complement_of_base_10_integer;
mod convert_1d_array_into_2d_array;
mod coordinate;
mod count_number_of_teams;
mod count_the_number_of_consistent_strings;
mod crawler_log_folder;
mod design_circular_deque;
mod divide_players_into_teams_of_equal_skill;
mod filling_bookcase_shelves;
mod find_common_characters;
mod find_the_safest_path_in_a_grid;
mod height_checker;
mod kth_distinct_string_in_an_array;
mod kth_largest_element_in_a_stream;
mod lemonad_change;
mod list;
mod longest_palindrome_by_concatenating_two_letter_words;
mod make_two_arrays_equal_by_reversing_subarrays;
mod maximum_distance_in_arrays;
mod merge_two_sorted_list;
mod minimum_bit_flips_to_convert_number;
mod minimum_cost_to_hire_k_workers;
mod minimum_deletions_to_make_string_balanced;
mod minimum_number_of_moves_to_seat_everyone;
mod minimum_number_of_pushes_to_type_word_ii;
mod my_calendar_i;
mod number_complement;
mod number_of_senior_citizens;
mod pass_the_pillow;
mod rank_transform_of_an_array;
mod relative_ranks;
mod relative_sort_array;
mod remove_nodes_from_linked_list;
mod remove_nth_node_from_end_of_list;
mod reverse_substrings_between_each_pair_of_parentheses;
mod score_after_clipping_matrix;
mod sort_array_by_increasing_frequency;
mod sort_the_jumbled_numbers;
mod sort_the_people;
mod tree;
mod two_keys_keyboard;
mod uncommon_words_from_two_sentences;
mod water_bottles;
mod x_of_a_kind_in_a_deck_of_cards;

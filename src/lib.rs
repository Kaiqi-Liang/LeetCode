#[macro_export]
macro_rules! counter {
    ($iter:expr) => {
        $iter.fold(HashMap::new(), |mut a, c| {
            *a.entry(c).or_insert(0) += 1;
            a
        })
    };
}

#[cfg(test)]
mod append_characters_to_string_to_make_subsequence;
#[cfg(test)]
mod average_waiting_time;
#[cfg(test)]
mod binary_tree_postorder_traversal;
#[cfg(test)]
mod check_if_a_word_occurs_as_a_prefix_of_any_word_in_a_sentence;
#[cfg(test)]
mod check_if_n_and_its_double_exist;
#[cfg(test)]
mod combination_sum;
#[cfg(test)]
mod combination_sum_ii;
#[cfg(test)]
mod complement_of_base_10_integer;
#[cfg(test)]
mod convert_1d_array_into_2d_array;
#[cfg(test)]
mod coordinate;
#[cfg(test)]
mod count_number_of_teams;
#[cfg(test)]
mod count_symmetric_integers;
#[cfg(test)]
mod count_the_number_of_consistent_strings;
#[cfg(test)]
mod count_vowel_substrings_of_a_string;
#[cfg(test)]
mod crawler_log_folder;
#[cfg(test)]
mod daily_temperatures;
#[cfg(test)]
mod defuse_the_bomb;
#[cfg(test)]
mod design_circular_deque;
#[cfg(test)]
mod divide_players_into_teams_of_equal_skill;
#[cfg(test)]
mod filling_bookcase_shelves;
#[cfg(test)]
mod find_common_characters;
#[cfg(test)]
mod find_the_safest_path_in_a_grid;
#[cfg(test)]
mod greatest_common_divisor_of_strings;
#[cfg(test)]
mod height_checker;
#[cfg(test)]
mod image_smoother;
#[cfg(test)]
mod kth_distinct_string_in_an_array;
#[cfg(test)]
mod kth_largest_element_in_a_stream;
#[cfg(test)]
mod lemonad_change;
#[cfg(test)]
mod list;
#[cfg(test)]
mod longest_palindrome_by_concatenating_two_letter_words;
#[cfg(test)]
mod make_two_arrays_equal_by_reversing_subarrays;
#[cfg(test)]
mod maximum_distance_in_arrays;
#[cfg(test)]
mod maximum_sum_of_distinct_subarrays_with_length_k;
#[cfg(test)]
mod merge_two_sorted_list;
#[cfg(test)]
mod minimum_bit_flips_to_convert_number;
#[cfg(test)]
mod minimum_cost_to_hire_k_workers;
#[cfg(test)]
mod minimum_deletions_to_make_string_balanced;
#[cfg(test)]
mod minimum_number_of_moves_to_seat_everyone;
#[cfg(test)]
mod minimum_number_of_pushes_to_type_word_ii;
#[cfg(test)]
mod my_calendar_i;
#[cfg(test)]
mod number_complement;
#[cfg(test)]
mod number_of_senior_citizens;
#[cfg(test)]
mod pass_the_pillow;
#[cfg(test)]
mod rank_transform_of_an_array;
#[cfg(test)]
mod relative_ranks;
#[cfg(test)]
mod relative_sort_array;
#[cfg(test)]
mod remove_duplicates_from_sorted_array;
#[cfg(test)]
mod remove_element;
#[cfg(test)]
mod remove_nodes_from_linked_list;
#[cfg(test)]
mod remove_nth_node_from_end_of_list;
#[cfg(test)]
mod reverse_substrings_between_each_pair_of_parentheses;
#[cfg(test)]
mod score_after_clipping_matrix;
#[cfg(test)]
mod shortest_subarray_with_sum_at_least_k;
#[cfg(test)]
mod sort_array_by_increasing_frequency;
#[cfg(test)]
mod sort_the_jumbled_numbers;
#[cfg(test)]
mod sort_the_people;
#[cfg(test)]
mod take_gifts_from_the_richest_pile;
#[cfg(test)]
mod tree;
#[cfg(test)]
mod two_keys_keyboard;
#[cfg(test)]
mod uncommon_words_from_two_sentences;
#[cfg(test)]
mod water_bottles;
#[cfg(test)]
mod x_of_a_kind_in_a_deck_of_cards;

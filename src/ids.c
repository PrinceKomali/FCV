#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

const uint64_t MOD_ROLE = 1136048449570209915;
const uint64_t OWNER_ROLE = 1136047797192380437;

const uint64_t ROLES_CHANNEL = 1136321118349828196;
const uint64_t TESTING_CHANNEL = 1136124445891768330;

const uint64_t NOTIF_ROLE = 1136439826829287424;
const uint64_t RED_ROLE = 1136694061286293637;
const uint64_t ORANGE_ROLE = 1136694101421588660;
const uint64_t YELLOW_ROLE = 1136694125295587398;
const uint64_t GREEN_ROLE = 1136694148666249296;
const uint64_t BLUE_ROLE = 1136694202739200163;
const uint64_t PURPLE_ROLE = 1136694229054271558;

bool is_color(uint64_t c) {
    return c == RED_ROLE || c == ORANGE_ROLE || c == YELLOW_ROLE || c == GREEN_ROLE || c == BLUE_ROLE || c == PURPLE_ROLE;
}
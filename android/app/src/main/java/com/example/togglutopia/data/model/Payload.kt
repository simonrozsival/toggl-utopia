package com.example.togglutopia.data.model

data class Payload(
        val projects: List<Project>,
        val time_entries: List<TimeEntry>,
        val user: User
)
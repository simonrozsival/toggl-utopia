package com.example.togglutopia.data.model

data class TimeEntry(
    val at: String,
    val description: String,
    val duration: Int,
    val id: Int,
    val project_id: Int?,
    val server_deleted_at: Any,
    val start: String
)
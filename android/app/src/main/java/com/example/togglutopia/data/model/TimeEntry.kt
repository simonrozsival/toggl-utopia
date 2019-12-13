package com.example.togglutopia.data.model

data class TimeEntry(
        val id: Int,
        val at: String,
        val description: String,
        val duration: Int,
        val project_id: Int?,
        val server_deleted_at: String?,
        val start: String,
        val workspace_id: Int,
        val edited: Boolean = false
)
package com.example.togglutopia.data.model

data class Project(
    val active: Boolean,
    val at: String,
    val color: String,
    val id: Int,
    val name: String,
    val server_deleted_at: Any
)
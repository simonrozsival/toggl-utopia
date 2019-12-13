package com.example.togglutopia.data.model

data class User(
    val api_token: String,
    val at: String,
    val fullname: String,
    val id: Int,
    val default_workspace_id: Int?
)
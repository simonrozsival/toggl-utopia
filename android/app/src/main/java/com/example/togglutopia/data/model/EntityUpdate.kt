package com.example.togglutopia.data.model

data class EntityUpdate<T>(
    val type: String,
    val payload: T
)
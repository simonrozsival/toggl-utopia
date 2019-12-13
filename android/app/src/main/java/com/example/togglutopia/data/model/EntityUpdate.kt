package com.example.togglutopia.data.model

data class EntityUpdate<T>(
    val type: UpdateType,
    val entity: T
)

enum class UpdateType {
    Changed, Created, Deleted
}
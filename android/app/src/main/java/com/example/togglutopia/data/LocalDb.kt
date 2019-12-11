package com.example.togglutopia.data

import android.content.Context
import android.content.SharedPreferences
import android.icu.lang.UCharacter.GraphemeClusterBreak.T
import androidx.core.content.edit
import com.example.togglutopia.data.model.Project
import com.example.togglutopia.data.model.TimeEntry
import com.example.togglutopia.data.model.User
import com.squareup.moshi.Moshi
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.channelFlow
import kotlinx.coroutines.flow.flowOn
import kotlin.coroutines.CoroutineContext


//@ExperimentalCoroutinesApi
//class LocalDb(context: Context) {
//
//    private val sharedPreferences = context.getSharedPreferences("utopia", Context.MODE_PRIVATE)
//    private val moshi = Moshi.Builder().build()
//    private val userAdapter = moshi.adapter(User::class.java)
//    private val projectAdapter = moshi.adapter(Project::class.java)
//    private val timeEntryAdapter = moshi.adapter(TimeEntry::class.java)
//
//    var user: User
//        get() = userAdapter.fromJson(sharedPreferences.getString("user", null)) ?: anonUser()
//        set(value) {
//            sharedPreferences.edit {
//                putString("user", userAdapter.toJson(value))
//            }
//        }
//    val projects: List<Project> = listOf()
//    val timeEntries: List<TimeEntry> = listOf()
//
//    fun userFlow(): Flow<User> {
//        return sharedPreferences.observeKey("user", anonUser())
//    }
//
//    private fun anonUser() = User("", "", "", 0)
//}
//
//@ExperimentalCoroutinesApi
//inline fun <reified T> SharedPreferences.observeKey(key: String, default: T, dispatcher: CoroutineContext = Dispatchers.Default): Flow<T> {
//    val flow: Flow<T> = channelFlow {
//        offer(getItem(key, default))
//
//        val listener = SharedPreferences.OnSharedPreferenceChangeListener { _, k ->
//            if (key == k) {
//                offer(getItem(key, default)!!)
//            }
//        }
//
//        registerOnSharedPreferenceChangeListener(listener)
//        awaitClose { unregisterOnSharedPreferenceChangeListener(listener) }
//    }
//    return flow.flowOn(dispatcher)
//}
//
//inline fun <reified T> SharedPreferences.getItem(key: String, default: T): T {
//    @Suppress("UNCHECKED_CAST")
//    return when (default){
//        is String -> getString(key, default) as T
//        is Int -> getInt(key, default) as T
//        is Long -> getLong(key, default) as T
//        is Boolean -> getBoolean(key, default) as T
//        is Float -> getFloat(key, default) as T
//        is Set<*> -> getStringSet(key, default as Set<String>) as T
//        is MutableSet<*> -> getStringSet(key, default as MutableSet<String>) as T
//        else -> throw IllegalArgumentException("generic type not handle ${T::class.java.name}")
//    }
//}
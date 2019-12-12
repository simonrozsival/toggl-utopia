package com.example.togglutopia.ui

import androidx.lifecycle.*
import com.example.togglutopia.data.Repository
import kotlinx.coroutines.ExperimentalCoroutinesApi

class MainViewModel : ViewModel(), MainInteractions {

    private val repository = Repository()

    override fun login(username: String, password: String) {
        repository.login(username, password)
    }

    override fun sync() {
        repository.sync()
    }
}

interface MainInteractions : LoginInteractions, LogInteractions

interface LoginInteractions {
    fun login(username: String, password: String)
}

interface LogInteractions {
    fun sync()
}
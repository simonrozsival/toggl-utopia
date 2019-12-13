package com.example.togglutopia.ui

import android.app.Application
import android.util.Log
import androidx.lifecycle.*
import com.example.togglutopia.data.Repository

class MainViewModel(app: Application) : AndroidViewModel(app), MainInteractions {

    private val repository = Repository(getApplication())

    init {
        repository.restoreState()
    }

    override fun login(username: String, password: String) {
        repository.login(username, password)
    }

    override fun sync() {
        repository.sync()
    }

    override fun onCleared() {
        super.onCleared()
        repository.persistState()
    }
}

interface MainInteractions : LoginInteractions, LogInteractions

interface LoginInteractions {
    fun login(username: String, password: String)
}

interface LogInteractions {
    fun sync()
}
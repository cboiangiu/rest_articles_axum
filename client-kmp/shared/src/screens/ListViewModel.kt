package com.jetbrains.kmpapp.screens

import com.jetbrains.kmpapp.data.ArticleObject
import com.jetbrains.kmpapp.data.ArticleRepository
import com.rickclephas.kmm.viewmodel.KMMViewModel
import com.rickclephas.kmm.viewmodel.stateIn
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow

class ListViewModel(articleRepository: ArticleRepository) : KMMViewModel() {
    val objects: StateFlow<List<ArticleObject>> =
        articleRepository.getObjects()
            .stateIn(viewModelScope, SharingStarted.WhileSubscribed(5000), emptyList())
}

package com.jetbrains.kmpapp.screens

import com.jetbrains.kmpapp.data.ArticleObject
import com.jetbrains.kmpapp.data.ArticleRepository
import com.rickclephas.kmm.viewmodel.KMMViewModel
import kotlinx.coroutines.flow.Flow

class DetailViewModel(private val articleRepository: ArticleRepository) : KMMViewModel() {
    fun getObject(objectId: String): Flow<ArticleObject?> =
        articleRepository.getObjectById(objectId)
}

package com.jetbrains.kmpapp.data

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.launch

class ArticleRepository(
    private val articleApi: ArticleApi,
    private val articleStorage: ArticleStorage,
    ) {
    private val scope = CoroutineScope(SupervisorJob())

    fun initialize() {
        scope.launch {
            refresh()
        }
    }

    suspend fun refresh() {
        articleStorage.saveObjects(articleApi.getData())
    }

    fun getObjects(): Flow<List<ArticleObject>> = articleStorage.getObjects()

    fun getObjectById(objectId: String): Flow<ArticleObject?> = articleStorage.getObjectById(objectId)
}

package com.jetbrains.kmpapp.data

import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.map

interface ArticleStorage {
    suspend fun saveObjects(newObjects: List<ArticleObject>)

    fun getObjectById(objectId: String): Flow<ArticleObject?>

    fun getObjects(): Flow<List<ArticleObject>>
}

class InMemoryArticleStorage : ArticleStorage {
    private val storedObjects = MutableStateFlow(emptyList<ArticleObject>())

    override suspend fun saveObjects(newObjects: List<ArticleObject>) {
        storedObjects.value = newObjects
    }

    override fun getObjectById(objectId: String): Flow<ArticleObject?> {
        return storedObjects.map { objects ->
            objects.find { it.id == objectId }
        }
    }

    override fun getObjects(): Flow<List<ArticleObject>> = storedObjects
}

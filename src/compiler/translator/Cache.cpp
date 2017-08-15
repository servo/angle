//
// Copyright (c) 2015 The ANGLE Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//

// Cache.cpp: Implements a cache for various commonly created objects.

#include <limits>

#include "common/angleutils.h"
#include "common/debug.h"
#include "compiler/translator/Cache.h"

namespace sh
{

namespace
{

class TScopedAllocator : angle::NonCopyable
{
  public:
    TScopedAllocator(TPoolAllocator *allocator) : mPreviousAllocator(GetGlobalPoolAllocator())
    {
        SetGlobalPoolAllocator(allocator);
    }
    ~TScopedAllocator() { SetGlobalPoolAllocator(mPreviousAllocator); }

  private:
    TPoolAllocator *mPreviousAllocator;
};

}  // namespace

TCache::TypeKey::TypeKey(TBasicType basicType,
                         TPrecision precision,
                         TQualifier qualifier,
                         unsigned char primarySize,
                         unsigned char secondarySize)
{
    static_assert(sizeof(components) <= sizeof(value), "TypeKey::value is too small");

    const size_t MaxEnumValue = std::numeric_limits<EnumComponentType>::max();

    // TODO: change to static_assert() once we deprecate MSVC 2013 support
    ASSERT(MaxEnumValue >= EbtLast && MaxEnumValue >= EbpLast && MaxEnumValue >= EvqLast &&
           "TypeKey::EnumComponentType is too small");

    value                    = 0;
    components.basicType     = static_cast<EnumComponentType>(basicType);
    components.precision     = static_cast<EnumComponentType>(precision);
    components.qualifier     = static_cast<EnumComponentType>(qualifier);
    components.primarySize   = primarySize;
    components.secondarySize = secondarySize;
}

TCache *TCache::sCache = nullptr;

namespace {
// I can't believe I'm writing this
struct CacheMutex {
#if defined(ANGLE_PLATFORM_WINDOWS)
    CRITICAL_SECTION mCS;
    CacheMutex() {
        ::InitializeCriticalSection(&mCS);
    }
    ~CacheMutex() {
        ::DeleteCriticalSection(&mCS);
    }
    void Lock() {
        EnterCriticalSection(&mCS);
    }
    void Unlock() {
        LeaveCriticalSection(&mCS);
    }

#elif defined(ANGLE_PLATFORM_POSIX)
    pthread_mutex_t mMutex;
    CacheMutex() {
        pthread_mutex_init(&mMutex, nullptr);
    }
    void Lock() {
        pthread_mutex_lock(&mMutex);
    }
    void Unlock() {
        pthread_mutex_unlock(&mMutex);
    }

#else
#error Need to know how to work with mutexes on this platform
#endif
};

CacheMutex sCacheMutex;

struct AutoEnterCacheMutex {
    AutoEnterCacheMutex() {
        sCacheMutex.Lock();
    }
    ~AutoEnterCacheMutex() {
        sCacheMutex.Unlock();
    }
};

}

void TCache::initialize()
{
    AutoEnterCacheMutex lock;
    if (sCache == nullptr)
    {
        sCache = new TCache();
    }
}

void TCache::destroy()
{
    AutoEnterCacheMutex lock;
    SafeDelete(sCache);
}

const TType *TCache::getType(TBasicType basicType,
                             TPrecision precision,
                             TQualifier qualifier,
                             unsigned char primarySize,
                             unsigned char secondarySize)
{
    TypeKey key(basicType, precision, qualifier, primarySize, secondarySize);
    auto it = sCache->mTypes.find(key);
    if (it != sCache->mTypes.end())
    {
        return it->second;
    }

    TScopedAllocator scopedAllocator(&sCache->mAllocator);

    TType *type = new TType(basicType, precision, qualifier, primarySize, secondarySize);
    type->realize();
    sCache->mTypes.insert(std::make_pair(key, type));

    return type;
}

}  // namespace sh

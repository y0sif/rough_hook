/****************************************************************************
** Meta object code from reading C++ file 'gamemanager.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/lib/src/gamemanager.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'gamemanager.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_GameManager_t {
    QByteArrayData data[15];
    char stringdata0[148];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_GameManager_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_GameManager_t qt_meta_stringdata_GameManager = {
    {
QT_MOC_LITERAL(0, 0, 11), // "GameManager"
QT_MOC_LITERAL(1, 12, 11), // "gameStarted"
QT_MOC_LITERAL(2, 24, 0), // ""
QT_MOC_LITERAL(3, 25, 10), // "ChessGame*"
QT_MOC_LITERAL(4, 36, 4), // "game"
QT_MOC_LITERAL(5, 41, 13), // "gameDestroyed"
QT_MOC_LITERAL(6, 55, 5), // "ready"
QT_MOC_LITERAL(7, 61, 8), // "finished"
QT_MOC_LITERAL(8, 70, 12), // "debugMessage"
QT_MOC_LITERAL(9, 83, 4), // "data"
QT_MOC_LITERAL(10, 88, 6), // "finish"
QT_MOC_LITERAL(11, 95, 13), // "onThreadReady"
QT_MOC_LITERAL(12, 109, 12), // "onThreadQuit"
QT_MOC_LITERAL(13, 122, 17), // "onGameInitialized"
QT_MOC_LITERAL(14, 140, 7) // "success"

    },
    "GameManager\0gameStarted\0\0ChessGame*\0"
    "game\0gameDestroyed\0ready\0finished\0"
    "debugMessage\0data\0finish\0onThreadReady\0"
    "onThreadQuit\0onGameInitialized\0success"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_GameManager[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       9,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       5,       // signalCount

 // signals: name, argc, parameters, tag, flags
       1,    1,   59,    2, 0x06 /* Public */,
       5,    1,   62,    2, 0x06 /* Public */,
       6,    0,   65,    2, 0x06 /* Public */,
       7,    0,   66,    2, 0x06 /* Public */,
       8,    1,   67,    2, 0x06 /* Public */,

 // slots: name, argc, parameters, tag, flags
      10,    0,   70,    2, 0x0a /* Public */,
      11,    0,   71,    2, 0x08 /* Private */,
      12,    0,   72,    2, 0x08 /* Private */,
      13,    1,   73,    2, 0x08 /* Private */,

 // signals: parameters
    QMetaType::Void, 0x80000000 | 3,    4,
    QMetaType::Void, 0x80000000 | 3,    4,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void, QMetaType::QString,    9,

 // slots: parameters
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void, QMetaType::Bool,   14,

       0        // eod
};

void GameManager::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<GameManager *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->gameStarted((*reinterpret_cast< ChessGame*(*)>(_a[1]))); break;
        case 1: _t->gameDestroyed((*reinterpret_cast< ChessGame*(*)>(_a[1]))); break;
        case 2: _t->ready(); break;
        case 3: _t->finished(); break;
        case 4: _t->debugMessage((*reinterpret_cast< const QString(*)>(_a[1]))); break;
        case 5: _t->finish(); break;
        case 6: _t->onThreadReady(); break;
        case 7: _t->onThreadQuit(); break;
        case 8: _t->onGameInitialized((*reinterpret_cast< bool(*)>(_a[1]))); break;
        default: ;
        }
    } else if (_c == QMetaObject::IndexOfMethod) {
        int *result = reinterpret_cast<int *>(_a[0]);
        {
            using _t = void (GameManager::*)(ChessGame * );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&GameManager::gameStarted)) {
                *result = 0;
                return;
            }
        }
        {
            using _t = void (GameManager::*)(ChessGame * );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&GameManager::gameDestroyed)) {
                *result = 1;
                return;
            }
        }
        {
            using _t = void (GameManager::*)();
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&GameManager::ready)) {
                *result = 2;
                return;
            }
        }
        {
            using _t = void (GameManager::*)();
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&GameManager::finished)) {
                *result = 3;
                return;
            }
        }
        {
            using _t = void (GameManager::*)(const QString & );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&GameManager::debugMessage)) {
                *result = 4;
                return;
            }
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject GameManager::staticMetaObject = { {
    QMetaObject::SuperData::link<QObject::staticMetaObject>(),
    qt_meta_stringdata_GameManager.data,
    qt_meta_data_GameManager,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *GameManager::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *GameManager::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_GameManager.stringdata0))
        return static_cast<void*>(this);
    return QObject::qt_metacast(_clname);
}

int GameManager::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QObject::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 9)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 9;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 9)
            *reinterpret_cast<int*>(_a[0]) = -1;
        _id -= 9;
    }
    return _id;
}

// SIGNAL 0
void GameManager::gameStarted(ChessGame * _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 0, _a);
}

// SIGNAL 1
void GameManager::gameDestroyed(ChessGame * _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 1, _a);
}

// SIGNAL 2
void GameManager::ready()
{
    QMetaObject::activate(this, &staticMetaObject, 2, nullptr);
}

// SIGNAL 3
void GameManager::finished()
{
    QMetaObject::activate(this, &staticMetaObject, 3, nullptr);
}

// SIGNAL 4
void GameManager::debugMessage(const QString & _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 4, _a);
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE

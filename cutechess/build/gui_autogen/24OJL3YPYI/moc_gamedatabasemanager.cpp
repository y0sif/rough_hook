/****************************************************************************
** Meta object code from reading C++ file 'gamedatabasemanager.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/gui/src/gamedatabasemanager.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'gamedatabasemanager.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_GameDatabaseManager_t {
    QByteArrayData data[13];
    char stringdata0[173];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_GameDatabaseManager_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_GameDatabaseManager_t qt_meta_stringdata_GameDatabaseManager = {
    {
QT_MOC_LITERAL(0, 0, 19), // "GameDatabaseManager"
QT_MOC_LITERAL(1, 20, 13), // "databaseAdded"
QT_MOC_LITERAL(2, 34, 0), // ""
QT_MOC_LITERAL(3, 35, 5), // "index"
QT_MOC_LITERAL(4, 41, 24), // "databaseAboutToBeRemoved"
QT_MOC_LITERAL(5, 66, 14), // "databasesReset"
QT_MOC_LITERAL(6, 81, 11), // "addDatabase"
QT_MOC_LITERAL(7, 93, 12), // "PgnDatabase*"
QT_MOC_LITERAL(8, 106, 8), // "database"
QT_MOC_LITERAL(9, 115, 14), // "removeDatabase"
QT_MOC_LITERAL(10, 130, 19), // "importDatabaseAgain"
QT_MOC_LITERAL(11, 150, 13), // "importPgnFile"
QT_MOC_LITERAL(12, 164, 8) // "fileName"

    },
    "GameDatabaseManager\0databaseAdded\0\0"
    "index\0databaseAboutToBeRemoved\0"
    "databasesReset\0addDatabase\0PgnDatabase*\0"
    "database\0removeDatabase\0importDatabaseAgain\0"
    "importPgnFile\0fileName"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_GameDatabaseManager[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       7,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       3,       // signalCount

 // signals: name, argc, parameters, tag, flags
       1,    1,   49,    2, 0x06 /* Public */,
       4,    1,   52,    2, 0x06 /* Public */,
       5,    0,   55,    2, 0x06 /* Public */,

 // slots: name, argc, parameters, tag, flags
       6,    1,   56,    2, 0x0a /* Public */,
       9,    1,   59,    2, 0x0a /* Public */,
      10,    1,   62,    2, 0x0a /* Public */,
      11,    1,   65,    2, 0x0a /* Public */,

 // signals: parameters
    QMetaType::Void, QMetaType::Int,    3,
    QMetaType::Void, QMetaType::Int,    3,
    QMetaType::Void,

 // slots: parameters
    QMetaType::Void, 0x80000000 | 7,    8,
    QMetaType::Void, QMetaType::Int,    3,
    QMetaType::Void, QMetaType::Int,    3,
    QMetaType::Void, QMetaType::QString,   12,

       0        // eod
};

void GameDatabaseManager::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<GameDatabaseManager *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->databaseAdded((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 1: _t->databaseAboutToBeRemoved((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 2: _t->databasesReset(); break;
        case 3: _t->addDatabase((*reinterpret_cast< PgnDatabase*(*)>(_a[1]))); break;
        case 4: _t->removeDatabase((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 5: _t->importDatabaseAgain((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 6: _t->importPgnFile((*reinterpret_cast< const QString(*)>(_a[1]))); break;
        default: ;
        }
    } else if (_c == QMetaObject::IndexOfMethod) {
        int *result = reinterpret_cast<int *>(_a[0]);
        {
            using _t = void (GameDatabaseManager::*)(int );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&GameDatabaseManager::databaseAdded)) {
                *result = 0;
                return;
            }
        }
        {
            using _t = void (GameDatabaseManager::*)(int );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&GameDatabaseManager::databaseAboutToBeRemoved)) {
                *result = 1;
                return;
            }
        }
        {
            using _t = void (GameDatabaseManager::*)();
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&GameDatabaseManager::databasesReset)) {
                *result = 2;
                return;
            }
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject GameDatabaseManager::staticMetaObject = { {
    QMetaObject::SuperData::link<QObject::staticMetaObject>(),
    qt_meta_stringdata_GameDatabaseManager.data,
    qt_meta_data_GameDatabaseManager,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *GameDatabaseManager::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *GameDatabaseManager::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_GameDatabaseManager.stringdata0))
        return static_cast<void*>(this);
    return QObject::qt_metacast(_clname);
}

int GameDatabaseManager::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QObject::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 7)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 7;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 7)
            *reinterpret_cast<int*>(_a[0]) = -1;
        _id -= 7;
    }
    return _id;
}

// SIGNAL 0
void GameDatabaseManager::databaseAdded(int _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 0, _a);
}

// SIGNAL 1
void GameDatabaseManager::databaseAboutToBeRemoved(int _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 1, _a);
}

// SIGNAL 2
void GameDatabaseManager::databasesReset()
{
    QMetaObject::activate(this, &staticMetaObject, 2, nullptr);
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE

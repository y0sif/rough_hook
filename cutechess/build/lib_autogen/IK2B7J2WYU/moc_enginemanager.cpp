/****************************************************************************
** Meta object code from reading C++ file 'enginemanager.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/lib/src/enginemanager.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'enginemanager.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_EngineManager_t {
    QByteArrayData data[7];
    char stringdata0[83];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_EngineManager_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_EngineManager_t qt_meta_stringdata_EngineManager = {
    {
QT_MOC_LITERAL(0, 0, 13), // "EngineManager"
QT_MOC_LITERAL(1, 14, 12), // "enginesReset"
QT_MOC_LITERAL(2, 27, 0), // ""
QT_MOC_LITERAL(3, 28, 11), // "engineAdded"
QT_MOC_LITERAL(4, 40, 5), // "index"
QT_MOC_LITERAL(5, 46, 22), // "engineAboutToBeRemoved"
QT_MOC_LITERAL(6, 69, 13) // "engineUpdated"

    },
    "EngineManager\0enginesReset\0\0engineAdded\0"
    "index\0engineAboutToBeRemoved\0engineUpdated"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_EngineManager[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       4,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       4,       // signalCount

 // signals: name, argc, parameters, tag, flags
       1,    0,   34,    2, 0x06 /* Public */,
       3,    1,   35,    2, 0x06 /* Public */,
       5,    1,   38,    2, 0x06 /* Public */,
       6,    1,   41,    2, 0x06 /* Public */,

 // signals: parameters
    QMetaType::Void,
    QMetaType::Void, QMetaType::Int,    4,
    QMetaType::Void, QMetaType::Int,    4,
    QMetaType::Void, QMetaType::Int,    4,

       0        // eod
};

void EngineManager::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<EngineManager *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->enginesReset(); break;
        case 1: _t->engineAdded((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 2: _t->engineAboutToBeRemoved((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 3: _t->engineUpdated((*reinterpret_cast< int(*)>(_a[1]))); break;
        default: ;
        }
    } else if (_c == QMetaObject::IndexOfMethod) {
        int *result = reinterpret_cast<int *>(_a[0]);
        {
            using _t = void (EngineManager::*)();
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&EngineManager::enginesReset)) {
                *result = 0;
                return;
            }
        }
        {
            using _t = void (EngineManager::*)(int );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&EngineManager::engineAdded)) {
                *result = 1;
                return;
            }
        }
        {
            using _t = void (EngineManager::*)(int );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&EngineManager::engineAboutToBeRemoved)) {
                *result = 2;
                return;
            }
        }
        {
            using _t = void (EngineManager::*)(int );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&EngineManager::engineUpdated)) {
                *result = 3;
                return;
            }
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject EngineManager::staticMetaObject = { {
    QMetaObject::SuperData::link<QObject::staticMetaObject>(),
    qt_meta_stringdata_EngineManager.data,
    qt_meta_data_EngineManager,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *EngineManager::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *EngineManager::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_EngineManager.stringdata0))
        return static_cast<void*>(this);
    return QObject::qt_metacast(_clname);
}

int EngineManager::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QObject::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 4)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 4;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 4)
            *reinterpret_cast<int*>(_a[0]) = -1;
        _id -= 4;
    }
    return _id;
}

// SIGNAL 0
void EngineManager::enginesReset()
{
    QMetaObject::activate(this, &staticMetaObject, 0, nullptr);
}

// SIGNAL 1
void EngineManager::engineAdded(int _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 1, _a);
}

// SIGNAL 2
void EngineManager::engineAboutToBeRemoved(int _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 2, _a);
}

// SIGNAL 3
void EngineManager::engineUpdated(int _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 3, _a);
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE

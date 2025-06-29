/****************************************************************************
** Meta object code from reading C++ file 'engineconfigurationmodel.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/gui/src/engineconfigurationmodel.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'engineconfigurationmodel.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_EngineConfigurationModel_t {
    QByteArrayData data[7];
    char stringdata0[102];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_EngineConfigurationModel_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_EngineConfigurationModel_t qt_meta_stringdata_EngineConfigurationModel = {
    {
QT_MOC_LITERAL(0, 0, 24), // "EngineConfigurationModel"
QT_MOC_LITERAL(1, 25, 13), // "onEngineAdded"
QT_MOC_LITERAL(2, 39, 0), // ""
QT_MOC_LITERAL(3, 40, 5), // "index"
QT_MOC_LITERAL(4, 46, 24), // "onEngineAboutToBeRemoved"
QT_MOC_LITERAL(5, 71, 15), // "onEngineUpdated"
QT_MOC_LITERAL(6, 87, 14) // "onEnginesReset"

    },
    "EngineConfigurationModel\0onEngineAdded\0"
    "\0index\0onEngineAboutToBeRemoved\0"
    "onEngineUpdated\0onEnginesReset"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_EngineConfigurationModel[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       4,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       0,       // signalCount

 // slots: name, argc, parameters, tag, flags
       1,    1,   34,    2, 0x08 /* Private */,
       4,    1,   37,    2, 0x08 /* Private */,
       5,    1,   40,    2, 0x08 /* Private */,
       6,    0,   43,    2, 0x08 /* Private */,

 // slots: parameters
    QMetaType::Void, QMetaType::Int,    3,
    QMetaType::Void, QMetaType::Int,    3,
    QMetaType::Void, QMetaType::Int,    3,
    QMetaType::Void,

       0        // eod
};

void EngineConfigurationModel::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<EngineConfigurationModel *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->onEngineAdded((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 1: _t->onEngineAboutToBeRemoved((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 2: _t->onEngineUpdated((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 3: _t->onEnginesReset(); break;
        default: ;
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject EngineConfigurationModel::staticMetaObject = { {
    QMetaObject::SuperData::link<QAbstractListModel::staticMetaObject>(),
    qt_meta_stringdata_EngineConfigurationModel.data,
    qt_meta_data_EngineConfigurationModel,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *EngineConfigurationModel::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *EngineConfigurationModel::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_EngineConfigurationModel.stringdata0))
        return static_cast<void*>(this);
    return QAbstractListModel::qt_metacast(_clname);
}

int EngineConfigurationModel::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QAbstractListModel::qt_metacall(_c, _id, _a);
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
QT_WARNING_POP
QT_END_MOC_NAMESPACE

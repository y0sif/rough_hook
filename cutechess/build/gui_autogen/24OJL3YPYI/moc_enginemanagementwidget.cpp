/****************************************************************************
** Meta object code from reading C++ file 'enginemanagementwidget.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/gui/src/enginemanagementwidget.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'enginemanagementwidget.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_EngineManagementWidget_t {
    QByteArrayData data[12];
    char stringdata0[143];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_EngineManagementWidget_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_EngineManagementWidget_t qt_meta_stringdata_EngineManagementWidget = {
    {
QT_MOC_LITERAL(0, 0, 22), // "EngineManagementWidget"
QT_MOC_LITERAL(1, 23, 8), // "updateUi"
QT_MOC_LITERAL(2, 32, 0), // ""
QT_MOC_LITERAL(3, 33, 12), // "updateSearch"
QT_MOC_LITERAL(4, 46, 5), // "terms"
QT_MOC_LITERAL(5, 52, 9), // "addEngine"
QT_MOC_LITERAL(6, 62, 15), // "configureEngine"
QT_MOC_LITERAL(7, 78, 11), // "QModelIndex"
QT_MOC_LITERAL(8, 90, 5), // "index"
QT_MOC_LITERAL(9, 96, 12), // "removeEngine"
QT_MOC_LITERAL(10, 109, 11), // "cloneEngine"
QT_MOC_LITERAL(11, 121, 21) // "browseDefaultLocation"

    },
    "EngineManagementWidget\0updateUi\0\0"
    "updateSearch\0terms\0addEngine\0"
    "configureEngine\0QModelIndex\0index\0"
    "removeEngine\0cloneEngine\0browseDefaultLocation"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_EngineManagementWidget[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       8,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       0,       // signalCount

 // slots: name, argc, parameters, tag, flags
       1,    0,   54,    2, 0x08 /* Private */,
       3,    1,   55,    2, 0x08 /* Private */,
       5,    0,   58,    2, 0x08 /* Private */,
       6,    0,   59,    2, 0x08 /* Private */,
       6,    1,   60,    2, 0x08 /* Private */,
       9,    0,   63,    2, 0x08 /* Private */,
      10,    0,   64,    2, 0x08 /* Private */,
      11,    0,   65,    2, 0x08 /* Private */,

 // slots: parameters
    QMetaType::Void,
    QMetaType::Void, QMetaType::QString,    4,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void, 0x80000000 | 7,    8,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,

       0        // eod
};

void EngineManagementWidget::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<EngineManagementWidget *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->updateUi(); break;
        case 1: _t->updateSearch((*reinterpret_cast< const QString(*)>(_a[1]))); break;
        case 2: _t->addEngine(); break;
        case 3: _t->configureEngine(); break;
        case 4: _t->configureEngine((*reinterpret_cast< const QModelIndex(*)>(_a[1]))); break;
        case 5: _t->removeEngine(); break;
        case 6: _t->cloneEngine(); break;
        case 7: _t->browseDefaultLocation(); break;
        default: ;
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject EngineManagementWidget::staticMetaObject = { {
    QMetaObject::SuperData::link<QWidget::staticMetaObject>(),
    qt_meta_stringdata_EngineManagementWidget.data,
    qt_meta_data_EngineManagementWidget,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *EngineManagementWidget::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *EngineManagementWidget::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_EngineManagementWidget.stringdata0))
        return static_cast<void*>(this);
    return QWidget::qt_metacast(_clname);
}

int EngineManagementWidget::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QWidget::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 8)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 8;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 8)
            *reinterpret_cast<int*>(_a[0]) = -1;
        _id -= 8;
    }
    return _id;
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE

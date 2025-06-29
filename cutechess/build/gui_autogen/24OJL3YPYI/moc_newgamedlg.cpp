/****************************************************************************
** Meta object code from reading C++ file 'newgamedlg.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/gui/src/newgamedlg.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'newgamedlg.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_NewGameDialog_t {
    QByteArrayData data[8];
    char stringdata0[90];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_NewGameDialog_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_NewGameDialog_t qt_meta_stringdata_NewGameDialog = {
    {
QT_MOC_LITERAL(0, 0, 13), // "NewGameDialog"
QT_MOC_LITERAL(1, 14, 15), // "configureEngine"
QT_MOC_LITERAL(2, 30, 0), // ""
QT_MOC_LITERAL(3, 31, 16), // "onVariantChanged"
QT_MOC_LITERAL(4, 48, 7), // "variant"
QT_MOC_LITERAL(5, 56, 15), // "onEngineChanged"
QT_MOC_LITERAL(6, 72, 5), // "index"
QT_MOC_LITERAL(7, 78, 11) // "Chess::Side"

    },
    "NewGameDialog\0configureEngine\0\0"
    "onVariantChanged\0variant\0onEngineChanged\0"
    "index\0Chess::Side"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_NewGameDialog[] = {

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
       1,    0,   34,    2, 0x08 /* Private */,
       3,    1,   35,    2, 0x08 /* Private */,
       5,    2,   38,    2, 0x08 /* Private */,
       5,    1,   43,    2, 0x28 /* Private | MethodCloned */,

 // slots: parameters
    QMetaType::Void,
    QMetaType::Void, QMetaType::QString,    4,
    QMetaType::Void, QMetaType::Int, 0x80000000 | 7,    6,    2,
    QMetaType::Void, QMetaType::Int,    6,

       0        // eod
};

void NewGameDialog::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<NewGameDialog *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->configureEngine(); break;
        case 1: _t->onVariantChanged((*reinterpret_cast< const QString(*)>(_a[1]))); break;
        case 2: _t->onEngineChanged((*reinterpret_cast< int(*)>(_a[1])),(*reinterpret_cast< Chess::Side(*)>(_a[2]))); break;
        case 3: _t->onEngineChanged((*reinterpret_cast< int(*)>(_a[1]))); break;
        default: ;
        }
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        switch (_id) {
        default: *reinterpret_cast<int*>(_a[0]) = -1; break;
        case 2:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 1:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< Chess::Side >(); break;
            }
            break;
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject NewGameDialog::staticMetaObject = { {
    QMetaObject::SuperData::link<QDialog::staticMetaObject>(),
    qt_meta_stringdata_NewGameDialog.data,
    qt_meta_data_NewGameDialog,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *NewGameDialog::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *NewGameDialog::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_NewGameDialog.stringdata0))
        return static_cast<void*>(this);
    return QDialog::qt_metacast(_clname);
}

int NewGameDialog::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QDialog::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 4)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 4;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 4)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 4;
    }
    return _id;
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE

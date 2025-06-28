/****************************************************************************
** Meta object code from reading C++ file 'importprogressdlg.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/gui/src/importprogressdlg.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'importprogressdlg.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_ImportProgressDialog_t {
    QByteArrayData data[9];
    char stringdata0[116];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_ImportProgressDialog_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_ImportProgressDialog_t qt_meta_stringdata_ImportProgressDialog = {
    {
QT_MOC_LITERAL(0, 0, 20), // "ImportProgressDialog"
QT_MOC_LITERAL(1, 21, 18), // "onImporterFinished"
QT_MOC_LITERAL(2, 40, 0), // ""
QT_MOC_LITERAL(3, 41, 13), // "onImportError"
QT_MOC_LITERAL(4, 55, 5), // "error"
QT_MOC_LITERAL(5, 61, 18), // "updateImportStatus"
QT_MOC_LITERAL(6, 80, 9), // "startTime"
QT_MOC_LITERAL(7, 90, 12), // "numReadGames"
QT_MOC_LITERAL(8, 103, 12) // "numReadBytes"

    },
    "ImportProgressDialog\0onImporterFinished\0"
    "\0onImportError\0error\0updateImportStatus\0"
    "startTime\0numReadGames\0numReadBytes"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_ImportProgressDialog[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       3,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       0,       // signalCount

 // slots: name, argc, parameters, tag, flags
       1,    0,   29,    2, 0x08 /* Private */,
       3,    1,   30,    2, 0x08 /* Private */,
       5,    3,   33,    2, 0x08 /* Private */,

 // slots: parameters
    QMetaType::Void,
    QMetaType::Void, QMetaType::Int,    4,
    QMetaType::Void, QMetaType::QTime, QMetaType::Int, QMetaType::LongLong,    6,    7,    8,

       0        // eod
};

void ImportProgressDialog::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<ImportProgressDialog *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->onImporterFinished(); break;
        case 1: _t->onImportError((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 2: _t->updateImportStatus((*reinterpret_cast< const QTime(*)>(_a[1])),(*reinterpret_cast< int(*)>(_a[2])),(*reinterpret_cast< qint64(*)>(_a[3]))); break;
        default: ;
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject ImportProgressDialog::staticMetaObject = { {
    QMetaObject::SuperData::link<QDialog::staticMetaObject>(),
    qt_meta_stringdata_ImportProgressDialog.data,
    qt_meta_data_ImportProgressDialog,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *ImportProgressDialog::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *ImportProgressDialog::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_ImportProgressDialog.stringdata0))
        return static_cast<void*>(this);
    return QDialog::qt_metacast(_clname);
}

int ImportProgressDialog::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QDialog::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 3)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 3;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 3)
            *reinterpret_cast<int*>(_a[0]) = -1;
        _id -= 3;
    }
    return _id;
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE

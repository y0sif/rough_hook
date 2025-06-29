/****************************************************************************
** Meta object code from reading C++ file 'pgnimporter.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/gui/src/pgnimporter.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'pgnimporter.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_PgnImporter_t {
    QByteArrayData data[9];
    char stringdata0[101];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_PgnImporter_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_PgnImporter_t qt_meta_stringdata_PgnImporter = {
    {
QT_MOC_LITERAL(0, 0, 11), // "PgnImporter"
QT_MOC_LITERAL(1, 12, 12), // "databaseRead"
QT_MOC_LITERAL(2, 25, 0), // ""
QT_MOC_LITERAL(3, 26, 12), // "PgnDatabase*"
QT_MOC_LITERAL(4, 39, 8), // "database"
QT_MOC_LITERAL(5, 48, 18), // "databaseReadStatus"
QT_MOC_LITERAL(6, 67, 7), // "started"
QT_MOC_LITERAL(7, 75, 12), // "numReadGames"
QT_MOC_LITERAL(8, 88, 12) // "numReadBytes"

    },
    "PgnImporter\0databaseRead\0\0PgnDatabase*\0"
    "database\0databaseReadStatus\0started\0"
    "numReadGames\0numReadBytes"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_PgnImporter[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       2,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       2,       // signalCount

 // signals: name, argc, parameters, tag, flags
       1,    1,   24,    2, 0x06 /* Public */,
       5,    3,   27,    2, 0x06 /* Public */,

 // signals: parameters
    QMetaType::Void, 0x80000000 | 3,    4,
    QMetaType::Void, QMetaType::QTime, QMetaType::Int, QMetaType::LongLong,    6,    7,    8,

       0        // eod
};

void PgnImporter::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<PgnImporter *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->databaseRead((*reinterpret_cast< PgnDatabase*(*)>(_a[1]))); break;
        case 1: _t->databaseReadStatus((*reinterpret_cast< const QTime(*)>(_a[1])),(*reinterpret_cast< int(*)>(_a[2])),(*reinterpret_cast< qint64(*)>(_a[3]))); break;
        default: ;
        }
    } else if (_c == QMetaObject::IndexOfMethod) {
        int *result = reinterpret_cast<int *>(_a[0]);
        {
            using _t = void (PgnImporter::*)(PgnDatabase * );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&PgnImporter::databaseRead)) {
                *result = 0;
                return;
            }
        }
        {
            using _t = void (PgnImporter::*)(const QTime & , int , qint64 );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&PgnImporter::databaseReadStatus)) {
                *result = 1;
                return;
            }
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject PgnImporter::staticMetaObject = { {
    QMetaObject::SuperData::link<Worker::staticMetaObject>(),
    qt_meta_stringdata_PgnImporter.data,
    qt_meta_data_PgnImporter,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *PgnImporter::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *PgnImporter::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_PgnImporter.stringdata0))
        return static_cast<void*>(this);
    return Worker::qt_metacast(_clname);
}

int PgnImporter::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = Worker::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 2)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 2;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 2)
            *reinterpret_cast<int*>(_a[0]) = -1;
        _id -= 2;
    }
    return _id;
}

// SIGNAL 0
void PgnImporter::databaseRead(PgnDatabase * _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 0, _a);
}

// SIGNAL 1
void PgnImporter::databaseReadStatus(const QTime & _t1, int _t2, qint64 _t3)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))), const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t2))), const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t3))) };
    QMetaObject::activate(this, &staticMetaObject, 1, _a);
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
